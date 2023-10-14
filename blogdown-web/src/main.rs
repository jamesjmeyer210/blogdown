use std::io::{Error, ErrorKind};
use blogdown_dao::{DaoContext, GroupCrud};
use blogdown_model::Group;

const DB_PATH: &'static str = "blogdown.db";

#[actix_web::main]
async fn main() -> Result<(),Error> {
    println!("Blogdown started...");

    let db_context = DaoContext::new(DB_PATH).await;
    let groups = db_context.groups();

    initialize_groups(&groups).await?;

    Ok(())
}

async fn initialize_groups(groups: &'_ GroupCrud<'_>) -> Result<(),Error> {
    let group_names = vec!["root", "admin", "blogger", "commenter"];
    for group_name in group_names {
        initialize_group(groups, group_name).await?;
    }
    Ok(())
}

async fn initialize_group(groups: &'_ GroupCrud<'_>, group_name: &str) -> Result<(),Error> {
    println!("Checking for {}", group_name);
    let x = groups.group_exists(group_name)
        .await
        .map_err(|e|Error::new(ErrorKind::InvalidData, e))?;
    match x {
        true => println!("{} exists", group_name),
        false => {
            groups.add_group(&Group::from(group_name))
                .await
                .map_err(|e|Error::new(ErrorKind::InvalidData, e))?;
            println!("added '{}' group", group_name);
        }
    }
    Ok(())
}