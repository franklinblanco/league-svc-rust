
// Here I'll have a lot of errors that repeat

use actix_web_utils::{dtos::message::MessageResource};
const PAGE_SIZE: u16 = 20;

/// Get From row and To row for database operations that are paged. 
/// ```
/// # use crate::util::repeat_utils::get_from_and_to_from_page;
/// let page_tuple = get_from_and_to_from_page(1).unwrap();
/// assert!(page_tuple.0 == 0);
/// assert!(page_tuple.1 == 20);
/// ```
pub fn get_from_and_to_from_page(page: u16) -> Result<(u32, u32), MessageResource> {
    if page == 0 { return Err(MessageResource::new_from_str("Page number cannot be 0."))}
    Ok((((page * PAGE_SIZE) - PAGE_SIZE).into(), (page * PAGE_SIZE).into()))
}