use crate::model::user_info::{
    UserInfoBy, UserInfoCreated, UserInfoForCreate, UserInfoForLogin, UserInfoRecord,
};
use lib_auth::pwd::{self, ContentToHash};
use uuid::Uuid;

use crate::{
    ctx::Ctx,
    model::{error::QueryError, Error, ModelManager, Result},
};

pub struct UserInfoBmc;

impl UserInfoBmc {
    pub async fn get<E>(_ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<E>
    where
        E: UserInfoBy,
    {
        let pool = mm.db();
        let sql = "SELECT TOP 1 * FROM dbo.user_info WHERE user_info_id=&1";
        let row = sqlx::query_as(sql)
            .bind(id.to_string())
            .fetch_one(pool)
            .await?;

        Ok(row)
    }

    pub async fn first_by_username<E>(_ctx: &Ctx, mm: &ModelManager, username: &str) -> Result<E>
    where
        E: UserInfoBy,
    {
        let pool = mm.db();
        let sql = "SELECT TOP 1 * FROM dbo.user_info WHERE username=&1";
        let row = sqlx::query_as(sql).bind(username).fetch_one(pool).await?;

        Ok(row)
    }

    pub async fn update_pwd(ctx: &Ctx, mm: &ModelManager, id: Uuid, password: &str) -> Result<()> {
        // let mut client = mm.db().get().await?;
        let pool = mm.db();
        let UserInfoForLogin { password_salt, .. } = Self::get(ctx, mm, id).await?;
        let password = pwd::hash_pwd(ContentToHash {
            content: password.to_string(),
            salt: password_salt,
        })
        .await?;

        // TODO: Create sp for update password
        let sql = "UPDATE dbo.user_info SET password=&2 WHERE user_info_id=&1;";
        let _row_affected = sqlx::query(sql)
            .bind(id)
            .bind(password)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(())
    }

    //     pub async fn create(
    //         ctx: &Ctx,
    //         mm: &ModelManager,
    //         user_info_for_create: UserInfoForCreate,
    //     ) -> Result<UserInfoRecord> {
    //         let UserInfoForCreate {
    //             Username,
    //             Email,
    //             Name,
    //             Password,
    //         } = user_info_for_create;

    //         let mut client = mm.db().get().await?;

    //         {
    //             let sql = "BEGIN TRANSACTION;";
    //             let _stream = client.simple_query(sql).await?;
    //         }

    //         let user_info_record = {
    //             let sql = "EXEC dbo.sp_userinfo_create @Username = @P1, @Name = @P2, @Email = @P3, @CreateBy = @P4;";
    //             let row = client
    //                 .query(sql, &[&Username, &Name, &Email, &ctx.user_id()])
    //                 .await?
    //                 .into_row()
    //                 .await?
    //                 .ok_or(Error::UserInfo(QueryError::NotReturnIDFromCreated));
    //             match row {
    //                 Ok(r) => UserInfoRecord::try_from_row(r)?,
    //                 Err(e) => {
    //                     let sql = "ROLLBACK;";
    //                     let _stream = client.simple_query(sql).await?;
    //                     return Err(e);
    //                 }
    //             }
    //         };

    //         let id = user_info_record
    //             .UserInfoID
    //             .ok_or(Error::UserInfo(QueryError::UserInfoRecordNotFound))?;

    //         let password = {
    //             let sql = "SELECT PasswordSalt FROM dbo.UserInfo WHERE UserInfoID = @P1;";
    //             let row = client
    //                 .query(sql, &[&id])
    //                 .await?
    //                 .into_row()
    //                 .await?
    //                 .ok_or(Error::UserInfo(QueryError::DataNotFound))?;
    //             let UserInfoCreated { PasswordSalt } = UserInfoCreated::try_from_row(row)?;

    //             let password = pwd::hash_pwd(ContentToHash {
    //                 content: Password.to_string(),
    //                 salt: PasswordSalt.unwrap_or_default(),
    //             })
    //             .await?;

    //             password
    //         };

    //         // TODO: Create sp for update password
    //         let sql = "UPDATE dbo.UserInfo SET [Password]=@P2 WHERE UserInfoID=@P1;";
    //         let row = client.query(sql, &[&id, &password]).await?.into_row().await;
    //         match row {
    //             Ok(_) => (),
    //             Err(e) => {
    //                 let sql = "ROLLBACK;";
    //                 let _stream = client.simple_query(sql).await?;
    //                 return Err(Error::Tiberius(e));
    //             }
    //         }

    //         {
    //             let sql = "COMMIT;";
    //             let _stream = client.simple_query(sql).await?;
    //         }

    //         Ok(user_info_record)
    //     }

    //     pub async fn list() -> () {
    //         todo!()
    //     }
}

// FIXME: change to postgres
// // region:    --- Tests
// #[cfg(test)]
// mod tests {
//     pub type Result<T> = core::result::Result<T, Error>;
//     pub type Error = Box<dyn std::error::Error>; // For tests.
//     use crate::model::{self, user_info::UserInfoForAuth};

//     use super::*;
//     use serial_test::serial;

//     #[serial]
//     #[tokio::test]
//     async fn test_first_ok_demo1() -> Result<()> {
//         // -- Setup & Fixtures
//         let mm = model::ModelManager::new().await?;
//         let ctx = Ctx::root_ctx();
//         let fx_username = "demo1";

//         // -- Exec
//         let user = UserInfoBmc::first_by_username::<UserInfoForAuth>(&ctx, &mm, fx_username)
//             .await?
//             .ok_or("Should have user 'demo1'")?;

//         // -- Check
//         assert_eq!(user.username, fx_username);

//         Ok(())
//     }
// }
// // endregion: --- Tests
