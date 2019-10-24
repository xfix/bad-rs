mod vec;

/// A single pass, no-nonsense sorting algorithm with O(n) complexity that
/// removes elements until the value is sorted.
///
/// This is a reference to [Joseph Stalin]'s horrible regime over Russia.
///
/// > Official records reveal 799,455 documented executions in the Soviet Union
/// > between 1921 and 1953; 681,692 of these were carried out between 1937 and
/// > 1938, the years of the [Great Purge].
///
/// # Source
///
/// <img src="https://i.redd.it/x9triplll1v11.jpg" width="360px">
///
/// <q>I came up with a single pass O(n) sort algorithm I call StalinSort. You
/// iterate down the list of elements checking if they're in order. Any element
/// which is out of order is eliminated. At the end you have a sorted list.</q>
/// <br>
/// <i>- mathew ✅ (@mathew@mastodon.social)</i>
///
/// [Joseph Stalin]: https://en.wikipedia.org/wiki/Joseph_Stalin
/// [Great Purge]:   https://en.wikipedia.org/wiki/Great_Purge
pub trait StalinSort {
    /// Removes unsorted elements until `self` is sorted.
    ///
    /// **Complexity:** O(n)
    fn stalin_sort(&mut self);

    /// Returns a copy of `self` that's sorted by removing unsorted elements.
    ///
    /// **Complexity:** O(n)
    #[inline]
    fn stalin_sorted(&self) -> Self where Self: Clone {
        let mut sorted = self.clone();
        sorted.stalin_sort();
        sorted
    }
}
