use records::recorder_client::RecorderClient;
use records::RecordRequest;
use tonic::Request;

pub mod records {
    tonic::include_proto!("records");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RecorderClient::connect("http://[::1]:50050").await?;
    let request = Request::new(
        RecordRequest{
            user_name: "Tech".to_string(),
            user_age: 20,
        }
    );

    let response = client.send_message(request).await?;

    println!("{:#?}", response);

    println!("Meta: {:#?}",response.metadata());
    println!("Message: {:#?}",response.get_ref());
    println!("Message: {:#?}",response.get_ref().message);
    Ok(())

}
