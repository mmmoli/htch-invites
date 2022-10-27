use super::traits::NotificationSerice;

pub struct AwsSes {
    // client: Client,
}

impl NotificationSerice for AwsSes {
    fn name(&self) -> String {
        "AWS SES".to_string()
    }

    fn send(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // struct MockAwsSes;

    // impl NotificationSerice for MockAwsSes {
    //     fn send(&self) -> anyhow::Result<()> {
    //         Ok(())
    //     }
    // }

    #[test]
    fn it_works() {
        let aws_ses = AwsSes {};
        aws_ses.send().unwrap();
    }
}
