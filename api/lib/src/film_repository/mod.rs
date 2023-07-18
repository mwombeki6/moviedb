pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

pub trait FilmRepository: Send + Sync + 'static {
    async fn get_film(&self) -> FilmResult<Vec<Film>>;
    async fn get_film(&self, id: &Uuid) -> FilmResult<Film>;
    async fn create_film(&self, id: &CreateFilm) -> FilmResult<Film>;
    async fn update_film(&self, id: &Film) -> FilmResult<Film>;
    async fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid>;
}