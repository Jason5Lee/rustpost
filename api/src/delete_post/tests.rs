use crate::common::*;
use actix_web::Result;

#[actix_rt::test]
async fn test_not_be_deleted_by_guest() {
    struct Mock;

    #[async_trait(?Send)]
    impl super::MockSteps for Mock {
        async fn is_admin(&self, _: AdminId) -> Result<bool> {
            Ok(false)
        }
        async fn get_post_creator(&self, _: PostId) -> Result<UserId> {
            Ok(UserId(1))
        }
        async fn delete_post(&self, _: PostId) -> Result<()> {
            panic!("post shouldn't be deleted by a guest")
        }
    }

    assert!(super::Steps(&Mock)
        .workflow(Identity::Guest, PostId(1))
        .await
        .is_err());
}

#[actix_rt::test]
async fn test_not_delete_by_not_creator() {
    struct Mock;

    #[async_trait(?Send)]
    impl super::MockSteps for Mock {
        async fn is_admin(&self, _: AdminId) -> Result<bool> {
            Ok(false)
        }
        async fn get_post_creator(&self, _: PostId) -> Result<UserId> {
            Ok(UserId(1))
        }
        async fn delete_post(&self, _: PostId) -> Result<()> {
            panic!("post shouldn't be deleted by an user that is not the creator")
        }
    }

    assert!(super::Steps(&Mock)
        .workflow(Identity::User(UserId(2)), PostId(1))
        .await
        .is_err());
}

#[actix_rt::test]
async fn test_deleted_by_creator() {
    struct Mock;

    #[async_trait(?Send)]
    impl super::MockSteps for Mock {
        async fn is_admin(&self, _: AdminId) -> Result<bool> {
            Ok(false)
        }
        async fn get_post_creator(&self, _: PostId) -> Result<UserId> {
            Ok(UserId(1))
        }
        async fn delete_post(&self, _: PostId) -> Result<()> {
            Ok(())
        }
    }

    assert!(super::Steps(&Mock)
        .workflow(Identity::User(UserId(1)), PostId(1))
        .await
        .is_ok());
}

#[actix_rt::test]
async fn test_deleted_by_admin() {
    struct Mock;

    #[async_trait(?Send)]
    impl super::MockSteps for Mock {
        async fn is_admin(&self, _: AdminId) -> Result<bool> {
            Ok(true)
        }
        async fn get_post_creator(&self, _: PostId) -> Result<UserId> {
            Ok(UserId(1))
        }
        async fn delete_post(&self, _: PostId) -> Result<()> {
            Ok(())
        }
    }

    assert!(super::Steps(&Mock)
        .workflow(Identity::Admin(AdminId(2)), PostId(1))
        .await
        .is_ok());
}
