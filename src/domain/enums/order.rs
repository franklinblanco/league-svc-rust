
/// Enum to signal from the client how they want their data.
/// Convention: "Name+Asc/Desc" Asc = Ascending order | Desc = Descending order.
pub enum OrderBy{
    RelevanceAsc,
    AlphabeticalAsc,
    TimeCreatedAsc,

    RelevanceDesc,
    AlphabeticalDesc,
    TimeCreatedDesc,
}