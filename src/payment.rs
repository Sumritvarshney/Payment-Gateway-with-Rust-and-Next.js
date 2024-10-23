use std::env;
use dotenv::dotenv;
use stripe::{CheckoutSession, Client, CreateCheckoutSession, CreateCheckoutSessionLineItems, CreatePrice, CreateProduct, Currency, IdOrCreate, Price, Product};

pub struct Payment {
    client: Client
}

impl Payment {
    pub fn new() -> Payment {
        dotenv().ok();

        let secret_key = env::var("STRIPE_KEY")
            .expect("Stripe Key is not set in the environment.");

        let client = Client::new(secret_key);

        Payment { client }
    }

    pub async fn create_stripe_session(&self) -> String {
        dotenv().ok();
        // Create a Product
        let product = {
            let create_product = CreateProduct::new("Test Product");
            Product::create(&self.client, create_product).await.unwrap()
        };

        // Create a Price for the Product
        let mut create_price = CreatePrice::new(Currency::USD);
        create_price.product = Some(IdOrCreate::Id(&product.id));
        create_price.unit_amount = Some(1000); // 1000 cents = $10
        create_price.expand = &["product"];

        let price = Price::create(&self.client, create_price).await.unwrap();

        // Fetch the Success and Failure URLs from environment variables
        let success_page = env::var("SUCCESS_URL")
            .expect("SUCCESS_URL is not set in the environment.");
        let failure_page = env::var("FAILURE_URL")
            .expect("FAILURE_URL is not set in the environment.");

        // Create a Checkout Session
        let mut session = CreateCheckoutSession::new();
        session.cancel_url = Some(&failure_page); // Pass the reference directly
        session.success_url = Some(&success_page); // Pass the reference directly
        session.mode = Some(stripe::CheckoutSessionMode::Payment);
        session.line_items = Some(vec![CreateCheckoutSessionLineItems {
            quantity: Some(4), // Set the quantity of items
            price: Some(price.id.to_string()), // Use the price ID from the created price
            ..Default::default()
        }]);
        session.expand = &["line_items.data.price.product"];

        // Create the Checkout Session with Stripe
        let checkout_session = CheckoutSession::create(&self.client, session)
            .await
            .expect("Failed to create Stripe session: missing required params");

        // Return the checkout session URL
        checkout_session.url.unwrap()
    }
}
