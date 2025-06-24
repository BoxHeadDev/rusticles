// 🦀 Rustlings Challenge: Encapsulation
//
// In this exercise, you'll implement encapsulation using a struct and methods.
// Your task is to implement the logic for a type that maintains a list of values
// and keeps track of their average.
//
// GOAL:
// - Prevent external code from accessing the internal data directly
// - Use `add`, `remove`, and `average` methods to control behavior
// - Use the `update_average` function internally to maintain the cached average

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    // Add a value to the collection and update the average
    pub fn add(&mut self, value: i32) {
        // TODO: push the value and update the average
    }

    // Remove a value from the collection (if any) and update the average
    pub fn remove(&mut self) -> Option<i32> {
        // TODO: pop the value and update the average only if Some
        None
    }

    // Return the current average
    pub fn average(&self) -> f64 {
        // TODO: return the average field
        0.0
    }

    // Recalculate the average based on current contents of `list`
    fn update_average(&mut self) {
        // TODO: set self.average to the average of the list values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_updates_correctly() {
        let mut stats = AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        };

        stats.add(10);
        stats.add(20);
        assert_eq!(stats.average(), 15.0);

        stats.remove();
        assert_eq!(stats.average(), 10.0);
    }
}
