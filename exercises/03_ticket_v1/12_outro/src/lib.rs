// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Default for Order {
    fn default() -> Self {
        Self {
            product_name: "product_name".to_string(),
            quantity: 1,
            unit_price: 1,
        }
    }
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        let mut v = Self::default();
        v.set_product_name(product_name);
        v.set_quantity(quantity);
        v.set_unit_price(unit_price);

        v
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn set_product_name(&mut self, v: String) {
        if v.is_empty() {
            panic!("emtpy product name");
        }
        if v.len() > 300 {
            panic!("product name too long");
        }
        self.product_name = v;
    }

    pub fn set_quantity(&mut self, v: u32) {
        if v == 0 {
            panic!("quantity must not be 0")
        }
        self.quantity = v;
    }

    pub fn set_unit_price(&mut self, v: u32) {
        if v == 0 {
            panic!("unit price must not be 0")
        }
        self.unit_price = v;
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }
}
