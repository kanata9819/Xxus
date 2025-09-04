use super::calc_hourly_wage::CalcHourlyWage;
use dioxus::prelude::*;
use shared_types::WorkRecord;
use tauri_sys::core::invoke;
use web_sys::console::log_1;

static CSS_PATH: Asset = asset!("/assets/styles.css");

#[derive(Clone)]
struct LocalSignals {
    pub date: Signal<String>,
    pub start_time: Signal<String>,
    pub end_time: Signal<String>,
    pub rest_time: Signal<String>,
    pub hourly_wage: Signal<String>,
    pub note: Signal<String>,
}

#[component]
pub fn WorkSchedule(
    on_submit: EventHandler<WorkRecord>,
    on_delete: EventHandler<String>,
    show_input: Signal<bool>,
    show_settings: Signal<bool>,
    timesheet_data_props: Vec<WorkRecord>,
    display_date_props: String,
) -> Element {
    // 画面表示内容シグナル
    let mut default_opt_sig: Signal<Option<WorkRecord>> = use_signal(|| None);
    let mut date: Signal<String> = use_signal(String::new);
    let mut start_time: Signal<String> = use_signal(String::new);
    let mut end_time: Signal<String> = use_signal(String::new);
    let mut rest_time: Signal<String> = use_signal(String::new);
    let mut hourly_wage: Signal<String> = use_signal(String::new);
    let mut note: Signal<String> = use_signal(String::new);
    let displaying_date: Signal<String> = use_signal(|| display_date_props.clone());
    // 内部処理用シグナル
    let mut error: Signal<String> = use_signal(String::new);
    let mut loading: Signal<bool> = use_signal(|| false);
    let mut initialized: Signal<bool> = use_signal(|| false);
    let mut is_data_exist: Signal<bool> = use_signal(|| false);
    // シグナルをまとめる
    let signals: LocalSignals = LocalSignals {
        date,
        start_time,
        end_time,
        rest_time,
        hourly_wage,
        note,
    };

    log_1(
        &format!(
            "WorkSchedule render: display_date={}",
            displaying_date.read()
        )
        .into(),
    );

    use_future(move || async move {
        let ini_result: bool =
            invoke::<bool>("init_default_value_db", &serde_json::json!({})).await;

        if ini_result && date.read().is_empty() {
            let fetched_default_opt: Option<WorkRecord> =
                invoke::<Option<WorkRecord>>("get_default_work_schedule", &serde_json::json!({}))
                    .await;
            default_opt_sig.set(fetched_default_opt);
        }
    });

    let time_sheet_data: Vec<WorkRecord> = timesheet_data_props.clone();

    use_effect(move || {
        if !*initialized.read() {
            if check_specific_data_exist(&time_sheet_data, displaying_date.read().to_string()) {
                is_data_exist.set(true);
                set_timesheet_data(
                    displaying_date.read().to_string(),
                    &time_sheet_data,
                    signals.clone(),
                );
            } else {
                is_data_exist.set(false);
                if let Some(default_data) = default_opt_sig.read().as_ref() {
                    set_default_data(
                        default_data,
                        signals.clone(),
                        displaying_date.read().to_string(),
                    );
                    initialized.set(true);
                }
            }
        }
    });

    use_effect(move || {
        if *initialized.read() && !*show_input.read() {
            initialized.set(false);
        }
    });

    let (minutes_opt, amount_opt) = {
        let minutes_opt = calc_minutes(&start_time(), &end_time(), &rest_time());
        let amount_opt = match (minutes_opt, parse_i32(&hourly_wage())) {
            (Some(mins), Some(wage)) if wage > 0 => Some(wage.saturating_mul(mins) / 60),
            _ => None,
        };
        (minutes_opt, amount_opt)
    };

    let is_exist_now =
        check_specific_data_exist(&timesheet_data_props, displaying_date.read().to_string());

    let (badge_class, badge_text) = if is_exist_now {
        (
            "px-3 py-1.5 text-[11px] font-semibold tracking-wide rounded-md
            bg-emerald-500/15 text-emerald-300 ring-1 ring-emerald-400/40
            shadow-inner shadow-emerald-900/40 backdrop-blur-sm
            flex items-center gap-1",
            "訂正",
        )
    } else {
        (
            "px-3 py-1.5 text-[11px] font-semibold tracking-wide rounded-md
            bg-rose-500/15 text-rose-300 ring-1 ring-rose-400/40
            shadow-inner shadow-rose-900/40 backdrop-blur-sm
            flex items-center gap-1",
            "新規",
        )
    };

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "modal-panel-dark flex flex-col items-center",
            header { class: "p-4 flex flex-row items-center gap-4",
                div { class: "{badge_class}",
                    // アイコン風ドット
                    span { class: "w-2 h-2 rounded-full bg-current opacity-70" }
                    span { "{badge_text}" }
                }
                // ページ全体の余白のみ（背景は周囲のダークに合わせる）
                div { class: "p-4 flex flex-row items-center gap-4",
                    "勤務実績"
                    button {
                        class: "px-4 py-2 rounded bg-blue-600 text-white",
                        onclick: move |_| { show_settings.set(true) },
                        "初期値設定"
                    }
                }
            }

            div { class: "min-h-[30vh] w-full p-4 flex flex-row gap-4 mx-auto z-30 inset-0",
                button { onclick: move |_| show_input.set(false), "×" }
                // ダーク調のパネル背景
                div { class: "flex flex-col gap-4 w-[50vw] max-w-[100vw]",
                    // エラー表示
                    if !error().is_empty() {
                        div { class: "text-red-600 text-sm", "{error()}" }
                    }
                    div { class: "flex flex-row gap-8",
                        // 日付
                        label { class: "font-medium", "日付" }
                        input {
                            class: "border p-2 rounded w-[15vw]",
                            r#type: "date",
                            value: date,
                            oninput: move |e| date.set(e.value()),
                        }
                    }
                    div { class: "flex flex-row gap-4",
                        // 開始時刻
                        label { class: "font-medium", "勤務時間" }
                        input {
                            class: "border p-2 rounded w-[13vw]",
                            r#type: "time",
                            value: start_time,
                            oninput: move |e| start_time.set(e.value()),
                        }
                        // 終了時刻
                        label { class: "font-medium", "~" }
                        input {
                            class: "border p-2 rounded w-[13vw]",
                            r#type: "time",
                            value: end_time,
                            oninput: move |e| end_time.set(e.value()),
                        }
                    }
                    div { class: "flex flex-row gap-4",
                        // 休憩時間
                        label { class: "font-medium", "休憩時間" }
                        input {
                            class: "border p-2 rounded w-[13vw]",
                            r#type: "time",
                            value: rest_time,
                            oninput: move |e| rest_time.set(e.value()),
                        }
                    }
                    div {
                        // 時給
                        label { class: "font-medium", "時給（円）" }
                        input {
                            class: "border p-2 rounded w-[15vw]",
                            r#type: "number",
                            inputmode: "numeric",
                            placeholder: "例: 1200",
                            value: hourly_wage,
                            oninput: move |e| hourly_wage.set(e.value()),
                        }
                    }

                    // 備考
                    label { class: "font-medium", "備考" }
                    textarea {
                        class: "border p-2 rounded min-h-[72px] resize-none",
                        placeholder: "メモ（任意）",
                        value: note,
                        oninput: move |e| note.set(e.value()),
                    }

                    // 自動計算の表示
                    div { class: "mt-2 text-sm opacity-80",
                        match minutes_opt {
                            Some(m) => rsx! {
                                div { "勤務時間: {format_minutes(m)}" }
                            },
                            None => rsx! {
                                div { "勤務時間: -" }
                            },
                        }
                        match amount_opt {
                            Some(a) => rsx! {
                                div { "概算支給額: {a} 円" }
                            },
                            None => rsx! {
                                div { "概算支給額: -" }
                            },
                        }
                    }

                    // アクション
                    div { class: "flex gap-2 mt-3",
                        button {
                            class: "px-3 py-2 rounded bg-gray-200 hover:bg-gray-300",
                            onclick: move |_| {
                                date.set(String::new());
                                start_time.set(String::new());
                                end_time.set(String::new());
                                hourly_wage.set(String::new());
                                rest_time.set(String::new());
                                note.set(String::new());
                                error.set(String::new());
                            },
                            "クリア"
                        }
                        //登録ボタン
                        button {
                            class: "px-3 py-2 rounded bg-blue-600 hover:bg-blue-700 text-white",
                            disabled: loading(),
                            onclick: move |_| {
                                if loading() {
                                    return;
                                }
                                loading.set(true);
                                match validate(
                                    &date.read(),
                                    &start_time.read(),
                                    &end_time.read(),
                                    &rest_time.read(),
                                    &hourly_wage.read(),
                                ) {
                                    Ok(()) => {
                                        let Some(minutes) = calc_minutes(
                                            &start_time.read(),
                                            &end_time.read(),
                                            &rest_time.read(),
                                        ) else {
                                            error
                                                .set(
                                                    "勤務時間の計算に失敗しました。（終了が開始より前？）"
                                                        .into(),
                                                );
                                            loading.set(false);
                                            return;
                                        };
                                        let Some(wage) = parse_i32(&hourly_wage()) else {
                                            error
                                                .set(
                                                    "時給は正の整数で入力してください。".into(),
                                                );
                                            loading.set(false);
                                            return;
                                        };
                                        let amount = wage.saturating_mul(minutes) / 60;
                                        let record = WorkRecord {
                                            date: date.read().to_string(),
                                            start_time: start_time.read().to_string(),
                                            end_time: end_time.read().to_string(),
                                            rest_time: rest_time.read().to_string(),
                                            hourly_wage: wage,
                                            minutes,
                                            amount,
                                            note: note.read().to_string(),
                                        };
                                        error.set(String::new());
                                        on_submit.call(record);
                                    }
                                    Err(msg) => error.set(msg),
                                }
                                loading.set(false);
                            },
                            if *loading.read() {
                                "保存中…"
                            } else {
                                "登録"
                            }
                        }
                        button {
                            class: "px-3 py-1.5 rounded-md bg-sky-600/80 hover:bg-sky-600 text-white text-xs font-semibold tracking-wide shadow transition",
                            onclick: move |_| {
                                if *loading.read() || !*is_data_exist.read() {
                                    error.set("削除できるデータがありません。".into());
                                    return;
                                }
                                loading.set(true);
                                on_delete.call(date.read().to_string());
                                loading.set(false);
                            },
                            "削除"
                        }
                    }
                }
                // }
                div {
                    // 時給計算コンポーネント
                    CalcHourlyWage {}
                }
            }
        }
    }
}

/// "HH:MM" を分に変換して差分を返す（終了 < 開始 は None）
fn calc_minutes(start: &str, end: &str, rest_time: &str) -> Option<i32> {
    let (sh, sm) = parse_hhmm(start)?;
    let (eh, em) = parse_hhmm(end)?;
    let (rh, rm) = parse_hhmm(rest_time)?;
    let s = sh * 60 + sm;
    let e = eh * 60 + em;
    (e >= s).then_some(e - s - (rh * 60 + rm))
}

/// "HH:MM" -> (時, 分)
fn parse_hhmm(s: &str) -> Option<(i32, i32)> {
    let mut it: std::str::Split<'_, char> = s.split(':');
    let h: i32 = it.next()?.parse().ok()?;
    let m: i32 = it.next()?.parse().ok()?;
    if (0..=23).contains(&h) && (0..=59).contains(&m) {
        Some((h, m))
    } else {
        None
    }
}

fn parse_i32(s: &str) -> Option<i32> {
    let t: &str = s.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<i32>().ok()
}

fn validate(date: &str, start: &str, end: &str, rest_time: &str, wage: &str) -> Result<(), String> {
    if date.trim().is_empty() {
        return Err("日付を入力してください。".into());
    }
    parse_hhmm(start).ok_or_else(|| "開始時間は HH:MM 形式で入力してください。".to_string())?;
    parse_hhmm(end).ok_or_else(|| "終了時間は HH:MM 形式で入力してください。".to_string())?;
    match parse_i32(wage) {
        Some(w) if w > 0 => {}
        _ => return Err("時給は正の整数で入力してください。".into()),
    }
    parse_hhmm(rest_time).ok_or_else(|| "休憩時間は HH:MM 形式で入力してください。".to_string())?;
    if calc_minutes(start, end, rest_time).is_none() {
        return Err("勤務時間が不正です。（終了は開始以降にしてください）".into());
    }
    Ok(())
}

fn format_minutes(m: i32) -> String {
    let h: i32 = m / 60;
    let mm: i32 = m % 60;
    if h > 0 {
        format!("{h}時間 {mm}分")
    } else {
        format!("{mm}分")
    }
}

fn set_timesheet_data(
    display_date: String,
    timesheet_data: &[WorkRecord],
    mut signals: LocalSignals,
) {
    for record in timesheet_data.iter() {
        if record.date == display_date {
            signals.date.set(record.date.clone());
            signals.start_time.set(record.start_time.clone());
            signals.end_time.set(record.end_time.clone());
            signals.rest_time.set(record.rest_time.clone());
            signals
                .hourly_wage
                .set(record.hourly_wage.clone().to_string());
            signals.note.set(record.note.clone());
            break;
        }
    }
}

fn check_specific_data_exist(timesheet_data: &[WorkRecord], display_date: String) -> bool {
    for record in timesheet_data.iter() {
        if record.date == display_date {
            return true;
        }
    }
    false
}

fn set_default_data(default_opt: &WorkRecord, mut signals: LocalSignals, display_date: String) {
    signals.date.set(display_date);
    signals.start_time.set(default_opt.start_time.clone());
    signals.end_time.set(default_opt.end_time.clone());
    signals.rest_time.set(default_opt.rest_time.clone());
    signals.hourly_wage.set(default_opt.hourly_wage.to_string());
    signals.note.set(default_opt.note.clone());
}
