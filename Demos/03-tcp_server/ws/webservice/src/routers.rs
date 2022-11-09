use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // scope 定义了一套资源的根路径
        .service(web::scope("courses")
        .route("/", web::post().to(new_course))
        .route("/{user_id}", web::get().to(get_courses_for_teacher))
        .route("/{user_id}/{course_id}", web::get().to(get_course_detail)));
}
