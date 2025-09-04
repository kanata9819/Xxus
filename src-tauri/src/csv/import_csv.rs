use encoding_rs::{SHIFT_JIS, UTF_8};
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::{
    fs::File,
    io::{BufReader, Read},
};

use csv::ReaderBuilder;
use shared_types::paypay::PayPayCsvRow; // ← あなたが追加した型名に合わせて直してね

/// 先頭バッファを見て UTF-8 っぽいか判定（BOM or 正当なUTF-8ならtrue）
fn sniff_is_utf8(buf: &[u8]) -> bool {
    buf.starts_with(&[0xEF, 0xBB, 0xBF]) || std::str::from_utf8(buf).is_ok()
}

/// UTF-8 / Shift_JIS を自動判定して UTF-8 に変換された Reader を返す
fn transcoded_reader(path: &str) -> Result<Box<dyn Read + Send>, String> {
    // 先頭だけ嗅ぐ
    let file: File = File::open(path).map_err(|e| format!("open error: {e}"))?;
    let mut sniff_reader: BufReader<File> = BufReader::new(file);
    let mut head: Vec<u8> = vec![0u8; 8192];
    let n: usize = sniff_reader
        .read(&mut head)
        .map_err(|e| format!("read error: {e}"))?;
    head.truncate(n);

    let enc: Option<&'static encoding_rs::Encoding> = if sniff_is_utf8(&head) {
        Some(UTF_8)
    } else {
        Some(SHIFT_JIS)
    };

    // 本読み込み用に再オープン
    let file: File = File::open(path).map_err(|e| format!("reopen error: {e}"))?;
    let reader = DecodeReaderBytesBuilder::new()
        .encoding(enc)
        .utf8_passthru(true) // 既にUTF-8なら余計な変換しない
        .build(file);

    Ok(Box::new(reader))
}

pub async fn import_csv(path: String) -> Result<bool, String> {
    println!("Importing CSV from path: {}", path);

    // 文字コードを気にせずUTF-8として読めるReaderにする
    let transcoded: Box<dyn Read + Send> = transcoded_reader(&path)?;

    // CSVリーダ設定：ヘッダーあり、列欠損に寛容、前後空白トリム
    let mut rdr: csv::Reader<Box<dyn Read + Send>> = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .trim(csv::Trim::Fields)
        .from_reader(transcoded);

    // まずヘッダーの確認（ログ用・デバッグしやすく）
    let headers: Vec<String> = rdr
        .headers()
        .map_err(|e| format!("header read error: {e}"))?
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    println!("[import_csv] headers = {:?}", headers);

    // 本体を一気に読み込む（必要ならここでDB保存や正規化に流せる）
    let mut count: usize = 0;
    for rec in rdr.deserialize() {
        let row: PayPayCsvRow = rec.map_err(|e| format!("row parse error: {e}"))?;
        // TODO: ここでDBにinsert / まとめてバルク挿入 など
        // db.insert_row(&row).await?;
        count += 1;
    }

    println!("[import_csv] imported {} rows ✅", count);
    Ok(true)
}
