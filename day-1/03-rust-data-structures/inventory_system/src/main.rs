// Product Inventory Management System
// Demonstrates structs, enums, HashMap, and Vec

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone, PartialEq)]
enum Category {
    Electronics,
    Clothing,
    Food,
    Books,
    Other,
}

impl Category {
    fn from_string(s: &str) -> Option<Category> {
        match s.to_lowercase().as_str() {
            "electronics" | "1" => Some(Category::Electronics),
            "clothing" | "2" => Some(Category::Clothing),
            "food" | "3" => Some(Category::Food),
            "books" | "4" => Some(Category::Books),
            "other" | "5" => Some(Category::Other),
            _ => None,
        }
    }
    
    fn to_string(&self) -> &str {
        match self {
            Category::Electronics => "Electronics",
            Category::Clothing => "Clothing",
            Category::Food => "Food",
            Category::Books => "Books",
            Category::Other => "Other",
        }
    }
}

#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    category: Category,
    price: f64,
    quantity: u32,
}

impl Product {
    fn new(id: u32, name: String, category: Category, price: f64, quantity: u32) -> Self {
        Product {
            id,
            name,
            category,
            price,
            quantity,
        }
    }
    
    fn total_value(&self) -> f64 {
        self.price * self.quantity as f64
    }
    
    fn is_in_stock(&self) -> bool {
        self.quantity > 0
    }
    
    fn display(&self) {
        println!("\n📦 Product #{}", self.id);
        println!("   Name: {}", self.name);
        println!("   Category: {}", self.category.to_string());
        println!("   Price: ${:.2}", self.price);
        println!("   Quantity: {}", self.quantity);
        println!("   Total Value: ${:.2}", self.total_value());
        println!("   Status: {}", if self.is_in_stock() { "✅ In Stock" } else { "❌ Out of Stock" });
    }
}

struct Inventory {
    products: HashMap<u32, Product>,
    next_id: u32,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            products: HashMap::new(),
            next_id: 1,
        }
    }
    
    fn add_product(&mut self, name: String, category: Category, price: f64, quantity: u32) -> u32 {
        let id = self.next_id;
        let product = Product::new(id, name, category, price, quantity);
        self.products.insert(id, product);
        self.next_id += 1;
        id
    }
    
    fn update_quantity(&mut self, id: u32, quantity: i32) -> Result<(), String> {
        match self.products.get_mut(&id) {
            Some(product) => {
                let new_quantity = product.quantity as i32 + quantity;
                if new_quantity < 0 {
                    return Err("Insufficient quantity".to_string());
                }
                product.quantity = new_quantity as u32;
                Ok(())
            }
            None => Err(format!("Product with ID {} not found", id)),
        }
    }
    
    fn get_product(&self, id: u32) -> Option<&Product> {
        self.products.get(&id)
    }
    
    fn search_by_name(&self, query: &str) -> Vec<&Product> {
        self.products
            .values()
            .filter(|p| p.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }
    
    fn get_by_category(&self, category: &Category) -> Vec<&Product> {
        self.products
            .values()
            .filter(|p| &p.category == category)
            .collect()
    }
    
    fn low_stock_items(&self, threshold: u32) -> Vec<&Product> {
        self.products
            .values()
            .filter(|p| p.quantity <= threshold)
            .collect()
    }
    
    fn total_inventory_value(&self) -> f64 {
        self.products.values().map(|p| p.total_value()).sum()
    }
    
    fn statistics(&self) {
        println!("\n📊 Inventory Statistics:");
        println!("   Total Products: {}", self.products.len());
        println!("   Total Inventory Value: ${:.2}", self.total_inventory_value());
        
        // Items in stock
        let in_stock = self.products.values().filter(|p| p.is_in_stock()).count();
        let out_of_stock = self.products.len() - in_stock;
        println!("   In Stock: {}", in_stock);
        println!("   Out of Stock: {}", out_of_stock);
        
        // Category breakdown
        let mut category_counts: HashMap<String, usize> = HashMap::new();
        for product in self.products.values() {
            *category_counts.entry(product.category.to_string().to_string()).or_insert(0) += 1;
        }
        
        println!("\n   Products by Category:");
        for (category, count) in category_counts {
            println!("      {}: {}", category, count);
        }
    }
}

fn main() {
    let mut inventory = Inventory::new();
    
    println!("=== Product Inventory Management System ===\n");
    
    loop {
        println!("\n1. Add Product");
        println!("2. Update Quantity");
        println!("3. View Product");
        println!("4. Search Products");
        println!("5. View by Category");
        println!("6. Low Stock Alert");
        println!("7. Inventory Statistics");
        println!("8. Exit");
        println!("Choose an option:");
        
        let choice = read_input().trim().to_string();
        
        match choice.as_str() {
            "1" => add_product(&mut inventory),
            "2" => update_quantity(&mut inventory),
            "3" => view_product(&inventory),
            "4" => search_products(&inventory),
            "5" => view_by_category(&inventory),
            "6" => low_stock_alert(&inventory),
            "7" => inventory.statistics(),
            "8" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn add_product(inventory: &mut Inventory) {
    println!("\nEnter product name:");
    let name = read_input().trim().to_string();
    
    println!("Select category:");
    println!("1. Electronics");
    println!("2. Clothing");
    println!("3. Food");
    println!("4. Books");
    println!("5. Other");
    
    let category = match Category::from_string(&read_input().trim()) {
        Some(c) => c,
        None => {
            println!("Invalid category!");
            return;
        }
    };
    
    println!("Enter price:");
    let price: f64 = match read_input().trim().parse() {
        Ok(p) => p,
        Err(_) => {
            println!("Invalid price!");
            return;
        }
    };
    
    println!("Enter quantity:");
    let quantity: u32 = match read_input().trim().parse() {
        Ok(q) => q,
        Err(_) => {
            println!("Invalid quantity!");
            return;
        }
    };
    
    let id = inventory.add_product(name, category, price, quantity);
    println!("✅ Product added with ID: {}", id);
}

fn update_quantity(inventory: &mut Inventory) {
    println!("\nEnter product ID:");
    let id: u32 = match read_input().trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid ID!");
            return;
        }
    };
    
    println!("Enter quantity change (positive to add, negative to remove):");
    let change: i32 = match read_input().trim().parse() {
        Ok(c) => c,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };
    
    match inventory.update_quantity(id, change) {
        Ok(_) => println!("✅ Quantity updated successfully!"),
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn view_product(inventory: &Inventory) {
    println!("\nEnter product ID:");
    let id: u32 = match read_input().trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Invalid ID!");
            return;
        }
    };
    
    match inventory.get_product(id) {
        Some(product) => product.display(),
        None => println!("Product not found!"),
    }
}

fn search_products(inventory: &Inventory) {
    println!("\nEnter search query:");
    let query = read_input().trim().to_string();
    
    let results = inventory.search_by_name(&query);
    
    if results.is_empty() {
        println!("No products found!");
    } else {
        println!("\n🔍 Search Results ({}):", results.len());
        for product in results {
            product.display();
        }
    }
}

fn view_by_category(inventory: &Inventory) {
    println!("\nSelect category:");
    println!("1. Electronics");
    println!("2. Clothing");
    println!("3. Food");
    println!("4. Books");
    println!("5. Other");
    
    let category = match Category::from_string(&read_input().trim()) {
        Some(c) => c,
        None => {
            println!("Invalid category!");
            return;
        }
    };
    
    let products = inventory.get_by_category(&category);
    
    if products.is_empty() {
        println!("No products in this category!");
    } else {
        println!("\n📂 {} Products ({}):", category.to_string(), products.len());
        for product in products {
            product.display();
        }
    }
}

fn low_stock_alert(inventory: &Inventory) {
    println!("\nEnter threshold quantity:");
    let threshold: u32 = match read_input().trim().parse() {
        Ok(t) => t,
        Err(_) => {
            println!("Invalid threshold!");
            return;
        }
    };
    
    let low_stock = inventory.low_stock_items(threshold);
    
    if low_stock.is_empty() {
        println!("✅ All products are well stocked!");
    } else {
        println!("\n⚠️  Low Stock Alert ({}):", low_stock.len());
        for product in low_stock {
            product.display();
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}
