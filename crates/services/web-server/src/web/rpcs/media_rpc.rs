// region: --- Modules

use lib_core::service::upload_media::CompleteUploadPayload;
use lib_core::service::upload_media::CompleteUploadResp;
use lib_core::service::upload_media::InitUploadPayload;
use lib_core::service::upload_media::InitUploadSessionResp;
use lib_core::service::upload_media::PartPresignedUrl;
use lib_core::service::upload_media::UploadMediaService;
use lib_core::service::upload_media::UploadPartPayload;
use lib_rpc::Result;
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_rpc::ParamsForCreate;
use lib_rpc::router::RpcRouter;
use lib_rpc::rpc_router;

// endregion: --- Modules

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		init_upload_media_session,
		generate_presigned_url_for_part,
        complete_upload_session
	)
}

pub async fn init_upload_media_session(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<InitUploadPayload>
) -> Result<InitUploadSessionResp> {
    let ParamsForCreate { data } = params;

    let resp = UploadMediaService::init_upload_session(&ctx, &mm, data).await?;

    Ok(resp)
}

pub async fn generate_presigned_url_for_part(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<UploadPartPayload>,
) -> Result<PartPresignedUrl> {

    let ParamsForCreate { data } = params;

    let resp: PartPresignedUrl = UploadMediaService::generate_presigned_url_for_part(&ctx, &mm, data).await?;

    Ok(resp)
}

pub async fn complete_upload_session(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<CompleteUploadPayload>,
) -> Result<CompleteUploadResp> {

    let ParamsForCreate { data } = params;

    let resp = UploadMediaService::complete_upload_session(&ctx, &mm, data).await?;

    Ok(resp)
}
