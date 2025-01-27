use super::pattern::Pattern;
use temp_reversi_core::Bitboard;

/// A manager to handle multiple patterns and their associated weights.
///
/// This structure manages a collection of patterns and their associated weights.
/// It provides functionality for adding, retrieving, filtering, and scoring patterns
/// based on the state of a Reversi game board.
pub struct PatternManager {
    /// A collection of patterns and their associated weights.
    ///
    /// Each entry consists of a `Pattern` and a corresponding weight (`f32`).
    /// Patterns contribute to the board evaluation based on their weights.
    patterns: Vec<(Pattern, f32)>, // (Pattern, Weight)
}

impl PatternManager {
    /// Creates a new, empty pattern manager.
    ///
    /// # Examples
    ///
    /// ```
    /// let manager = PatternManager::new();
    /// assert!(manager.all_patterns().is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    /// Adds a pattern with an associated weight to the manager.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to be added.
    /// * `weight` - The weight associated with the pattern. Higher weights indicate greater importance.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut manager = PatternManager::new();
    /// manager.add_pattern(Pattern::new(0x8100000000000081, Some("Corner")), 10.0);
    /// ```
    pub fn add_pattern(&mut self, pattern: Pattern, weight: f32) {
        self.patterns.push((pattern, weight));
    }

    /// Retrieves all patterns and their associated weights.
    ///
    /// # Returns
    ///
    /// A reference to the internal collection of patterns and weights.
    ///
    /// # Examples
    ///
    /// ```
    /// let manager = PatternManager::new();
    /// let patterns = manager.all_patterns();
    /// assert!(patterns.is_empty());
    /// ```
    pub fn all_patterns(&self) -> &Vec<(Pattern, f32)> {
        &self.patterns
    }

    /// Finds a pattern by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the pattern to search for.
    ///
    /// # Returns
    ///
    /// An optional reference to the pattern and its weight if found.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut manager = PatternManager::new();
    /// manager.add_pattern(Pattern::new(0x8100000000000081, Some("Corner")), 10.0);
    ///
    /// let corner = manager.find_by_name("Corner");
    /// assert!(corner.is_some());
    /// ```
    pub fn find_by_name(&self, name: &str) -> Option<&(Pattern, f32)> {
        self.patterns
            .iter()
            .find(|(p, _)| p.name.as_deref() == Some(name))
    }

    /// Filters patterns by a custom condition.
    ///
    /// # Arguments
    ///
    /// * `condition` - A closure that takes a reference to a `(Pattern, f32)` and returns a boolean.
    ///
    /// # Returns
    ///
    /// A vector of references to the patterns and weights that satisfy the condition.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut manager = PatternManager::new();
    /// manager.add_pattern(Pattern::new(0x8100000000000081, Some("Corner")), 10.0);
    /// manager.add_pattern(Pattern::new(0x7E8181818181817E, Some("Edge")), 5.0);
    ///
    /// let filtered = manager.filter_patterns(|(_, weight)| *weight >= 10.0);
    /// assert_eq!(filtered.len(), 1);
    /// ```
    pub fn filter_patterns<F>(&self, condition: F) -> Vec<&(Pattern, f32)>
    where
        F: Fn(&(Pattern, f32)) -> bool,
    {
        self.patterns.iter().filter(|p| condition(p)).collect()
    }

    /// Calculates the evaluation score for a given bitboard.
    ///
    /// This method checks each pattern against the provided `Bitboard` state.
    /// If a pattern matches the board, its weight is added to the total score.
    ///
    /// # Arguments
    ///
    /// * `bitboard` - The current game board state represented as a `Bitboard`.
    ///
    /// # Returns
    ///
    /// The total evaluation score as a `f32`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut manager = PatternManager::new();
    /// manager.add_pattern(Pattern::new(0x8100000000000081, Some("Corner")), 10.0);
    ///
    /// let bitboard = Bitboard::new(0x8100000000000081, 0);
    /// let score = manager.calculate_score(&bitboard);
    /// assert_eq!(score, 10.0);
    /// ```
    pub fn calculate_score(&self, bitboard: &Bitboard) -> f32 {
        self.patterns
            .iter()
            .map(|(pattern, weight)| {
                let matched = bitboard.bits().0 & pattern.board_mask;
                if matched != 0 {
                    *weight
                } else {
                    0.0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_reversi_core::Bitboard;

    /// Test adding and retrieving patterns.
    #[test]
    fn test_add_and_retrieve_patterns() {
        let mut manager = PatternManager::new();

        manager.add_pattern(Pattern::new(0x8100000000000081, Some("Corner")), 10.0);
        manager.add_pattern(Pattern::new(0x7E8181818181817E, Some("Edge")), 5.0);

        let all_patterns = manager.all_patterns();
        assert_eq!(all_patterns.len(), 2);

        let corner = manager.find_by_name("Corner");
        assert!(corner.is_some());
        assert_eq!(corner.unwrap().1, 10.0); // Check weight
    }

    /// Test filtering patterns by a custom condition.
    #[test]
    fn test_filter_patterns() {
        let mut manager = PatternManager::new();

        manager.add_pattern(Pattern::new(0x8100000000000081, Some("Corner")), 10.0);
        manager.add_pattern(Pattern::new(0x7E8181818181817E, Some("Edge")), 5.0);

        let filtered: Vec<&(Pattern, f32)> = manager.filter_patterns(|(_, weight)| *weight >= 10.0);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].0.name.as_deref(), Some("Corner"));
    }

    /// Test calculating the evaluation score for a given bitboard.
    #[test]
    fn test_calculate_score() {
        let mut manager = PatternManager::new();

        // Add patterns
        manager.add_pattern(Pattern::new(0x8100000000000081, Some("Corner")), 10.0); // Corners
        manager.add_pattern(Pattern::new(0x7E8181818181817E, Some("Edge")), 5.0); // Edges

        // Create a bitboard where only corners are occupied
        let bitboard = Bitboard::new(0x8100000000000081, 0);

        // Calculate score
        let score = manager.calculate_score(&bitboard);
        assert_eq!(score, 10.0); // Only "Corner" pattern matches
    }
}
