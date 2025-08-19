#[derive(Debug)]
struct Catalog{
    items: Vec<Media>
}

impl Catalog{
    fn new() -> Self{
        Catalog {items: vec![]}
    }

    fn add(&mut self, media: Media){
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        }else{
            None
        }
        
    }
