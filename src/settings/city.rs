use crate::error::GeolocationError;
use crate::geolocation::Place;
use tokio::sync::oneshot;

const MAX_QUERY_LENGTH: usize = 60;

pub type SearchResult = Result<Vec<Place>, GeolocationError>;

#[derive(Debug, Default)]
pub enum Status {
    #[default]
    Typing,
    Searching(oneshot::Receiver<SearchResult>),
    Found(Vec<Place>, usize),
    NotFound,
    NoConnection,
}

#[derive(Debug, Default)]
pub struct CitySearch {
    pub query: String,
    pub status: Status,
}

impl CitySearch {
    pub fn type_char(&mut self, ch: char) {
        if self.query.chars().count() < MAX_QUERY_LENGTH {
            self.query.push(ch);
            self.status = Status::Typing;
        }
    }

    pub fn erase(&mut self) {
        self.query.pop();
        self.status = Status::Typing;
    }

    pub fn select(&mut self, step: isize) {
        if let Status::Found(places, selected) = &mut self.status {
            *selected = selected
                .saturating_add_signed(step)
                .min(places.len().saturating_sub(1));
        }
    }

    pub fn chosen(&self) -> Option<&Place> {
        match &self.status {
            Status::Found(places, selected) => places.get(*selected),
            _ => None,
        }
    }

    pub fn query_to_search(&self) -> Option<&str> {
        let query = self.query.trim();
        (!query.is_empty() && !matches!(self.status, Status::Searching(_))).then_some(query)
    }

    pub fn wait(&mut self, receiver: oneshot::Receiver<SearchResult>) {
        self.status = Status::Searching(receiver);
    }

    pub fn receive(&mut self) {
        let Status::Searching(receiver) = &mut self.status else {
            return;
        };
        self.status = match receiver.try_recv() {
            Ok(Ok(places)) if places.is_empty() => Status::NotFound,
            Ok(Ok(places)) => Status::Found(places, 0),
            Ok(Err(_)) | Err(oneshot::error::TryRecvError::Closed) => Status::NoConnection,
            Err(oneshot::error::TryRecvError::Empty) => return,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::NetworkError;
    use crate::geolocation::GeoLocation;

    fn place(label: &str) -> Place {
        Place {
            label: label.to_string(),
            location: GeoLocation {
                latitude: 45.76,
                longitude: 4.83,
                city: Some("Lyon".to_string()),
            },
        }
    }

    fn answered(result: SearchResult) -> CitySearch {
        let mut search = CitySearch {
            query: "Lyon".to_string(),
            status: Status::Typing,
        };
        let (sender, receiver) = oneshot::channel();
        search.wait(receiver);
        assert_eq!(search.query_to_search(), None);
        search.receive();
        assert!(matches!(search.status, Status::Searching(_)));
        sender.send(result).unwrap();
        search.receive();
        search
    }

    #[test]
    fn test_results_are_picked_with_arrows_within_bounds() {
        let mut search = answered(Ok(vec![
            place("Lyon, Rhône, France"),
            place("Lyon, Mississippi"),
        ]));
        assert_eq!(search.chosen().unwrap().label, "Lyon, Rhône, France");
        search.select(1);
        search.select(1);
        assert_eq!(search.chosen().unwrap().label, "Lyon, Mississippi");
        search.select(-5);
        assert_eq!(search.chosen().unwrap().label, "Lyon, Rhône, France");
    }

    #[test]
    fn test_empty_answer_and_network_error_are_told_apart() {
        assert!(matches!(answered(Ok(Vec::new())).status, Status::NotFound));
        let error = GeolocationError::Unreachable(NetworkError::ConnectionRefused {
            url: "https://nominatim.openstreetmap.org/search".to_string(),
        });
        let search = answered(Err(error));
        assert!(matches!(search.status, Status::NoConnection));
        assert_eq!(search.query_to_search(), Some("Lyon"));
    }

    #[test]
    fn test_typing_drops_results_and_blank_query_is_not_searched() {
        let mut search = answered(Ok(vec![place("Lyon, Rhône, France")]));
        search.type_char('s');
        assert!(search.chosen().is_none());
        assert_eq!(search.query_to_search(), Some("Lyons"));

        let mut search = CitySearch::default();
        search.type_char(' ');
        assert_eq!(search.query_to_search(), None);
    }
}
