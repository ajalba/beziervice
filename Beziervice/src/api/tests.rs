#[cfg(test)]
mod actix_tests {

    use actix_web::test;
    use actix_web::{web, App};
    use serde_json::json;
    use crate::bezier_curves::curves::BezierCurve;
    
    use crate::api::handlers::*;

    #[actix_web::test]
    async fn test_get_simple_curve() {
        let mut test = test::init_service(
            App::new().service(
                web::resource("/").route(web::get().to(get_simple_curve)),
                
                )
        )
        .await;

        let request = test::TestRequest::get()
            .uri("/")
            .to_request();

        let response = test::call_service(&mut test, request).await;
        print!("{}", response.status());
        assert!(response.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_interpolate_curve() {
        let mut test = test::init_service(
            App::new().service(
                interpolate_function
            
            )
        )
        .await;

        let request = test::TestRequest::get()
            .uri("/interpolate_function/2")
            .to_request();

        let response = test::call_service(&mut test, request).await;
        print!("{}", response.status());
        assert!(response.status().is_success());
    }

    #[actix_web::test]
    async fn test_evaluate_curve() {
        let mut test = test::init_service(
            App::new().service(
                web::resource("/evaluate_curve").route(web::post().to(evaluate_simple_curve)),
                
                )
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/evaluate_curve")
            .set_json(json!(CurveBase{
                name: "generic".to_string(),
                points_x: vec![1.0, 2.0, 3.0],
                points_y: vec![0.0,4.0,5.0],
                points_z: vec![3.0, 5.0, 6.0]
            }))
            .to_request();

        let response = test::call_service(&mut test, request).await;
        assert!(response.status().is_success());
    }

    #[actix_web::test]
    async fn test_create_curve() {
        let mut test = test::init_service(
            App::new().service(
                web::resource("/create_curve").route(web::post().to(create_simple_curve)),
                
                )
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/create_curve")
            .set_json(json!(CurveBase{
                name: "generic".to_string(),
                points_x: vec![1.0, 2.0, 3.0],
                points_y: vec![0.0,4.0,5.0],
                points_z: vec![3.0, 5.0, 6.0]
            }))
            .to_request();

        let response = test::call_service(&mut test, request).await;
        assert!(response.status().is_success());
    }
}