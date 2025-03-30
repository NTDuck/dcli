pub struct Endpoints {
    pub scheme: &'static str,
    pub domain: &'static str,
    pub port: &'static str,

    pub task_router_path: &'static str,
    pub create_task_handler_path: &'static str,
    pub view_tasks_handler_path: &'static str,
}
