use dioxus::prelude::*;
use shared_types::WorkRecord;
use tauri_sys::core::invoke;

static CSS_PATH: Asset = asset!("assets/styles.css");

#[component]
pub fn SettingDefaultValue(on_submit: EventHandler<WorkRecord>) -> Element {
    let mut start_time: Signal<String> = use_signal(|| String::new());
    let mut end_time: Signal<String> = use_signal(|| String::new());
    let mut rest_time: Signal<String> = use_signal(|| String::new());
    let mut hourly_wage: Signal<String> = use_signal(|| String::new());
    let mut error: Signal<String> = use_signal(|| String::new());
    let mut loading: Signal<bool> = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            let default_opt: Option<WorkRecord> =
                invoke::<Option<WorkRecord>>("get_default_work_schedule", &serde_json::json!({}))
                    .await;
            if let Some(default) = default_opt {
                start_time.set(default.start_time);
                end_time.set(default.end_time);
                rest_time.set(default.rest_time);
                hourly_wage.set(default.hourly_wage.to_string());
            }
        });
    });

    let (minutes_opt, amount_opt) = {
        let minutes_opt = calc_minutes(&start_time(), &end_time(), &rest_time());
        let amount_opt = match (minutes_opt, parse_i32(&hourly_wage())) {
            (Some(mins), Some(wage)) if wage > 0 => Some(wage.saturating_mul(mins) / 60),
            _ => None,
        };
        (minutes_opt, amount_opt)
    };

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        div { class: "flex flex-col gap-4 w-[40vw]",
            // エラー表示
            if !error().is_empty() {
                div { class: "text-red-600 text-sm", "{error()}" }
            }

            // 勤務時間（開始〜終了）
            div { class: "flex flex-row gap-4",
                label { class: "font-medium", "勤務時間" }
                input {
                    class: "border p-2 rounded w-[13vw]",
                    r#type: "time",
                    value: start_time,
                    oninput: move |e| start_time.set(e.value()),
                }
                label { class: "font-medium", "~" }
                input {
                    class: "border p-2 rounded w-[13vw]",
                    r#type: "time",
                    value: end_time,
                    oninput: move |e| end_time.set(e.value()),
                }
            }

            // 休憩時間
            div { class: "flex flex-row gap-4",
                label { class: "font-medium", "休憩時間" }
                input {
                    class: "border p-2 rounded w-[13vw]",
                    r#type: "time",
                    value: rest_time,
                    oninput: move |e| rest_time.set(e.value()),
                }
            }

            // 時給
            div {
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
                        start_time.set(String::new());
                        end_time.set(String::new());
                        hourly_wage.set(String::new());
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
                        match validate(&start_time(), &end_time(), &rest_time(), &hourly_wage()) {
                            Ok(()) => {
                                let Some(minutes) = calc_minutes(
                                    &start_time(),
                                    &end_time(),
                                    &rest_time(),
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
                                    date: String::new(),
                                    start_time: start_time(),
                                    end_time: end_time(),
                                    rest_time: rest_time(),
                                    hourly_wage: wage,
                                    note: String::new(),
                                    minutes,
                                    amount,
                                };
                                error.set(String::new());
                                on_submit.call(record);
                            }
                            Err(msg) => error.set(msg),
                        }
                        loading.set(false);
                    },
                    if loading() {
                        "保存中…"
                    } else {
                        "登録"
                    }
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

fn validate(start: &str, end: &str, rest_time: &str, wage: &str) -> Result<(), String> {
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
