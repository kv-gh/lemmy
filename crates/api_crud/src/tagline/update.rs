use activitypub_federation::config::Data;
use actix_web::web::Json;
use chrono::Utc;
use lemmy_api_common::{
  context::LemmyContext,
  tagline::{TaglineResponse, UpdateTagline},
  utils::{get_url_blocklist, is_admin, local_site_to_slur_regex, process_markdown},
};
use lemmy_db_schema::{
  source::{
    local_site::LocalSite,
    tagline::{Tagline, TaglineUpdateForm},
  },
  traits::Crud,
};
use lemmy_db_views::structs::LocalUserView;
use lemmy_utils::error::LemmyError;

#[tracing::instrument(skip(context))]
pub async fn update_tagline(
  data: Json<UpdateTagline>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> Result<Json<TaglineResponse>, LemmyError> {
  // Make sure user is an admin
  is_admin(&local_user_view)?;

  let local_site = LocalSite::read(&mut context.pool()).await?;

  let slur_regex = local_site_to_slur_regex(&local_site);
  let url_blocklist = get_url_blocklist(&context).await?;
  let content = process_markdown(&data.content, &slur_regex, &url_blocklist, &context).await?;

  let tagline_form = TaglineUpdateForm {
    content,
    updated: Utc::now(),
  };

  let tagline = Tagline::update(&mut context.pool(), data.id, &tagline_form).await?;

  Ok(Json(TaglineResponse { tagline }))
}
