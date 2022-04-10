use dialoguer::{console::Term, theme::ColorfulTheme, Select};

mod actions;
mod config;
mod context;
mod tools;

fn main() {
    println!("Welcome to the ultimate Sultan CLI!");
    let mut context = context::Context::initialize();
    println!(
        "Current working directory: {}.",
        context.paths.dirs.working_dir.display()
    );
    println!("All necessary data will be placed here.");

    let mut actions = actions::build_actions_que();

    loop {
        let mut filtered_actions: Vec<(usize, &str)> = Vec::new();
        for (index, action) in actions.iter().enumerate() {
            if action.can_execute(&context) {
                filtered_actions.push((index, action.get_name()))
            }
        }

        let action_names: Vec<&str> = filtered_actions.iter().map(|action| action.1).collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&action_names)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        let index = filtered_actions[selection.unwrap()].0;
        actions[index].execute(&mut context);
    }
}
