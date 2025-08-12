use dioxus::prelude::*;
static CSS_PATH: Asset = asset!("/assets/styles.css");

#[component]
pub fn CalcHourlyWage() -> Element {
    let salary: Signal<String> = use_signal(|| String::new());
    let working_hours: Signal<String> = use_signal(|| String::new());
    let mut calculated_wage: Signal<String> = use_signal(|| String::new());

    use_effect(move || {
        let result: Result<String, Box<dyn std::error::Error>> =
            calc_hourly_wage(salary(), working_hours());
        match result {
            Ok(wage) => calculated_wage.set(wage),
            Err(_) => calculated_wage.set("入力してください".to_string()),
        }
    });

    rsx! {
        link { rel: "stylesheet", href: CSS_PATH }
        // 余白のみ（背景は周囲のダークに合わせる）
        div { class: "min-h-[40vh] p-4",
            // ダーク調のパネル
            div { class: "modal-panel-dark w-[20vw] max-w-[640px] flex flex-col gap-4 items-center",
                h2 { class: "font-bold text-xl", "時給計算" }

                // 勤務時間入力
                div { class: "flex flex-col gap-2",
                    label { class: "text-sm opacity-80", "勤務時間（合計時間[h]）" }
                    input {
                        class: "border p-2 rounded w-[10vw]",
                        r#type: "number",
                        value: working_hours,
                        oninput: move |e| handle_working_hours_change(working_hours, e.value()),
                    }
                }

                // 給与入力
                div { class: "flex flex-col gap-2",
                    label { class: "text-sm opacity-80", "給与（合計額）" }
                    input {
                        class: "border p-2 rounded w-[10vw]",
                        r#type: "number",
                        value: salary,
                        oninput: move |e| handle_salary_change(salary, e.value()),
                    }
                }

                // 計算結果表示
                div { class: "flex flex-col mt-2 text-sm opacity-90",
                    label { class: "text-sm opacity-80", "計算結果(円)" }
                    textarea {
                        class: "mt-1 px-3 py-2 rounded border w-[10vw]",
                        readonly: true,
                        resize: "none",
                        "{calculated_wage()}"
                    }
                }
            }
        }
    }
}

fn handle_salary_change(mut salary: Signal<String>, input: String) {
    // 入力値が空でない場合のみ更新
    if !input.is_empty() {
        //TODO いつかカンマフォーマットとかしたい
        salary.set(input);
    }
}

fn handle_working_hours_change(mut working_hours: Signal<String>, input: String) {
    if !input.is_empty() {
        working_hours.set(input);
    }
}

fn calc_hourly_wage(
    salary: String,
    working_hours: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let hourly_wage: i64;

    if salary.is_empty() || working_hours.is_empty() {
        return Err("error".into());
    }

    hourly_wage = salary.parse::<i64>()? / working_hours.parse::<i64>()?;

    Ok(hourly_wage.to_string())
}
