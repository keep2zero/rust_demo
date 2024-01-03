mod async_await {
   

    pub async fn down(url: &str) -> String {
        let result = reqwest::get(url).await;
        println!("result: {:?}", result);

        "".to_string()
    }
}

// use tokio::macros;
#[cfg(test)]
mod tests {
    use tokio::join;

    use crate::async_await::down;

    #[tokio::test]
    async fn test_down() {

        let w = down("https://www.wutongwei.com");

        let w1 = down("https://www.baidu.com");

        let w2 = down("https://www.qq.com");

        join!(w, w1, w2);
    }
}
