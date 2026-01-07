use tokio::sync::Mutex;

trait Payment {
    fn pay(&self);
}

struct Creaditcard;
struct Paypal;

impl Payment for Creaditcard {
    fn pay(&self) {
        println!("Pay with credit card");
    }
}

impl Payment for Paypal {
    fn pay(&self) {
        println!("Pay with paypal");
    }
}

#[cfg(test)]
mod test_box {
    use std::{rc::Rc, sync::{Arc, Mutex}, thread::{self}};

    use crate::_box_rc_arc::{Creaditcard, Payment, Paypal};
    

    #[test]
    fn test_arc() {
        let counter = Arc::new(Mutex::new(0));
        let handles: Vec<_> = (0..5).map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            })
        }).collect();

        for h in handles {
            h.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    #[test]
    fn pay_with_credit_card() {
        let cc_box: Box<dyn Payment> = Box::new(Creaditcard);
        cc_box.pay();

        // This Rc::new() counted as one strong_count
        let cc_rc: Rc<dyn Payment> = Rc::new(Creaditcard);

        let cc_rc_one: Rc<dyn Payment> = Rc::clone(&cc_rc);
        cc_rc_one.pay();

        let cc_rc_two: Rc<dyn Payment> = Rc::clone(&cc_rc);
        cc_rc_two.pay();

        println!("Rc strong_count {}", Rc::strong_count(&cc_rc))
    }

    #[test]
    fn pay_with_paypal() {
        let pp: Box<dyn Payment> = Box::new(Paypal);
        pp.pay();
    }
}
