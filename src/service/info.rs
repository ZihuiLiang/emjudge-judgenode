use actix_web::{get, post, web, HttpResponse};

use crate::{requests::LanguageRequest, responses::{AllInfo, AllInfoResponse, CpuInfo, CpuInfoResponse, DiskInfo, DiskInfoResponse, LanguageInfo, LanguageInfoResponse, LanguagesInfo, LanguagesInfoResponse, MemInfo, MemInfoResponse, SysInfo, SysInfoResponse, TestPressureResponse}, settings::Settings};



#[get("/sys")]
pub async fn sys() -> HttpResponse {
    let sys_info_response = SysInfoResponse::from(SysInfo::new());
    HttpResponse::Ok().json(sys_info_response)
}

#[get("/all")]
pub async fn all() -> HttpResponse {
    let all_info_response = AllInfoResponse::from(AllInfo::new());
    HttpResponse::Ok().json(all_info_response)
}


#[get("/cpu")]
pub async fn cpu() -> HttpResponse {
    let cpu_info_response = CpuInfoResponse::from(CpuInfo::new());
    HttpResponse::Ok().json(cpu_info_response)
}

#[get("/mem")]
pub async fn mem() -> HttpResponse {
    let mem_info_response = MemInfoResponse::from(MemInfo::new());
    HttpResponse::Ok().json(mem_info_response)
}

#[get("/disk")]
pub async fn disk() -> HttpResponse {
    let disk_info_response = DiskInfoResponse::from(DiskInfo::new());
    HttpResponse::Ok().json(disk_info_response)
}

#[get("/languages")]
pub async fn languages(settings: web::Data<Settings>) -> HttpResponse {
    let languages_info_response = LanguagesInfoResponse::from(LanguagesInfo::new(&settings.compile_and_exe));
    HttpResponse::Ok().json(languages_info_response)
}

#[post("/language")]
pub async fn language(settings: web::Data<Settings>, req: web::Json<LanguageRequest>) -> HttpResponse {
    let language_info_response = LanguageInfoResponse::from(LanguageInfo::new(&req.language, &settings.compile_and_exe));
    HttpResponse::Ok().json(language_info_response)
}

#[get("/test_pressure")]
pub async fn test_pressure(settings: web::Data<Settings>) -> HttpResponse {
    let test_pressure_response = TestPressureResponse::Ok(settings.get_test_pressure());
    HttpResponse::Ok().json(test_pressure_response)
}
