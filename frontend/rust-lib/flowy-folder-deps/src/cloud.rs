pub use anyhow::Error;
pub use collab_folder::{Folder, FolderData, Workspace};
use uuid::Uuid;

use lib_infra::future::FutureResult;

/// [FolderCloudService] represents the cloud service for folder.
pub trait FolderCloudService: Send + Sync + 'static {
  /// Creates a new workspace for the user.
  /// Returns error if the cloud service doesn't support multiple workspaces
  fn create_workspace(&self, uid: i64, name: &str) -> FutureResult<Workspace, Error>;

  fn open_workspace(&self, workspace_id: &str) -> FutureResult<(), Error>;

  /// Returns all workspaces of the user.
  /// Returns vec![] if the cloud service doesn't support multiple workspaces
  fn get_all_workspace(&self) -> FutureResult<Vec<WorkspaceRecord>, Error>;

  fn get_folder_data(
    &self,
    workspace_id: &str,
    uid: &i64,
  ) -> FutureResult<Option<FolderData>, Error>;

  fn get_folder_snapshots(
    &self,
    workspace_id: &str,
    limit: usize,
  ) -> FutureResult<Vec<FolderSnapshot>, Error>;

  fn get_folder_updates(&self, workspace_id: &str, uid: i64) -> FutureResult<Vec<Vec<u8>>, Error>;

  fn service_name(&self) -> String;
}

pub struct FolderSnapshot {
  pub snapshot_id: i64,
  pub database_id: String,
  pub data: Vec<u8>,
  pub created_at: i64,
}

pub fn gen_workspace_id() -> Uuid {
  uuid::Uuid::new_v4()
}

pub fn gen_view_id() -> Uuid {
  uuid::Uuid::new_v4()
}

#[derive(Debug)]
pub struct WorkspaceRecord {
  pub id: String,
  pub name: String,
  pub created_at: i64,
}
