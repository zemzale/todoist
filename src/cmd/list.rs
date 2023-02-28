pub fn list(filter: String, raw: boolean) {
    let resp = client
        .find(Some(api::TaskFilter {
            day_filter: Some(filter.to_owned().unwrap_or(String::from("today|overdue"))),
        }))
        .await;
    match resp {
        Ok(resp) => {
            let mut output_rows: Vec<Vec<String>> = Vec::new();

            for task in resp.iter() {
                let project_name = task.project(&client).await.unwrap().name;
                output_rows.push(vec![
                    task.id.to_owned(),
                    project_name,
                    task.content.to_owned(),
                    task.priority.to_string(),
                ]);
            }

            if raw.unwrap_or(false) {
                for row in output_rows {
                    for field in row {
                        print!("{},", field);
                    }
                    println!();
                }
            } else {
                let mut table = Table::new();
                table
                    .set_header(vec!["ID", "Project", "Task name", "Priority"])
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS);

                table.add_rows(output_rows);
                println!("{table}");
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
