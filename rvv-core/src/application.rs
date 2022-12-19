pub struct Application
{
    pub name: String
}

pub fn run(app: Application)
{
    println!("Running application: {}", app.name);
}