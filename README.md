# Symulator mrowiska

Program symuluje kolonie mrówek. Mrówki przemieszczają się w losowym kierunku oraz podnoszą i upuszczają przedmioty na planszy heksagonalnej.

Program wykorzystuje bibliotekę Raylib w celu wizualizacji mrowiska.

![image](https://github.com/bjaglowski/anthill/assets/89010497/0b3cf705-a620-4de8-97d2-db3db6627fa8)


## Uruchomienie
```shell
cargo run
# lub
cargo run [WYSOKOŚĆ PLANSZY] [SZEROKOŚĆ PLANSZY] [ILOŚĆ ITERACJI] [CZAS POMIĘDZY ITERACJAMI W MILISEKUNDACH]
```

## Zasady działania symulacji

- Plansza ma pewną ilość pól i nie posiada granic, tzn mrowisko jest skończone lecz nieograniczone (mrówka przemieszczająca się do krawędzi okna znajdzie się po jego drugiej stronie).
- Przedmioty dzielą się na:
  - liście oznaczone kolorem niebieskim
  - patyki oznaczone kolorem żółtym
- Mrówka (oznaczona kolorem czerwonym) w każdej iteracji symulacji przemieszcza się pięciokrotnie w losowych kierunkach, w przypadku planszy hexagonalnej - mrówka może przemieszczać się w sześciu kierunkach.
- Mrówka, która nie przenosi przedmiotu może wejść na pole z przedmiotem, aby go podnieść.
- Mrówka, która przenosi przedmiot zmienia swój kolor na pomarańczowy.
- Mrówka, nie może wejść na pole z innym przedmiotem, jeśli w danym momencie przenosi ona przedmiot. Może jednak wspołdzielić pole z inną mrówką.
- W kolejnej iteracji, jeśli mrówka sąsiaduje z przedmiotem o tym samym typie. Przymocowuje przedmiot na polu na którym stoi.
- Przedmiot jest przymocowywany do pola z siłą proporcjonalną do ilości sąsiadujących w danym momencie przedmiotów o tym samym typie.

## Funkcja main
- Inicjuje okno wyświetlane przy pomocy biblioteki raylib.
- Tworzy planszę i wypełnia ją mrówkami oraz przedmiotami.
- Uruchamia główną pętlę symulacji, aktualizując stan bytów i rysując planszę.
```rust
fn main() {
    // ...
}
```

## Komponenty
### struct Config
Struktura przechowująca konfigurację podaną w parametrach uruchomienia (lub domyślne).
```rust
pub struct Config {
    pub max_x: usize,
    pub max_y: usize,
    pub iterations: usize,
    pub interval: usize,
}
```


### enum ItemType
Typ enumeracyjny określający rodzaj przedmiotu
```rust
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ItemType {
    Lisc, // blue
    Kij, // yellow
}
```


### struct Item
Struktura opisująca przedmiot - jego położenie na planszy, typ oraz siłę z jaką został umieszczony na polu.
```rust
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Item {
    x: usize,
    y: usize,
    freeze_time_left: usize,
    item_type: ItemType,
}
```

### struct Ant
Struktura opisująca mrówkę - jej położenie na planszy oraz przenoszony przedmiot.
```rust
pub struct Ant { // red
    x: usize,
    y: usize,
    carrying_item: Option<Item>,
}
```

### struct Board
Struktura opisująca planszę - jej rozmiar, listę przedmiotów oraz mrówek.
```rust
pub struct Board {
    max_x: usize,
    max_y: usize,
    items: Vec<Item>,
    ants: Vec<Ant>,
}
```

## Funkcje i metody

### Ant
- new - tworzy nową mrówkę
- get_neighbors - zwraca pozycję sąsiadujących komórek.
```rust
impl Ant {
  fn new(x: usize, y: usize, max_x: usize, max_y: usize) -> Self {
    Self { x, y, max_x, max_y, carrying_item: None }
  }

  fn get_neighbors(&self) -> Vec<(usize, usize)> {
    // ...
  }
}
```

### Board
- new - tworzy planszę
- calculate_center_x oraz calculate_center_y - obliczają koordynaty środka heksagonu w celu wyświetlenia
- pick_or_leave - obsługuje logikę podnoszenia i upuszczania przedmiotów przez mrówki.
- draw - rysuje planszę
```rust
impl Board {
  fn new(x: usize, y: usize) -> Self {
    // ...
  }

  fn calculate_center_x(&self, x: usize, y: usize, radius: f32) -> f32 {
    // ...
  }

  fn calculate_center_y(&self, y: usize, radius: f32) -> f32 {
    // ...
  }

  fn pick_or_leave(&mut self) {
    // ...
  }

  fn draw(&self, d: &mut RaylibDrawHandle, radius: f32) {
    // ...
  }
}
```

## Inne funkcje
### 
- draw_hexagon - rysuje heksagon na podadej pozycji, o podanym rozmiarze i kolorze
```rust
fn draw_hexagon(d: &mut RaylibDrawHandle, x: f32, y: f32, radius: f32, color: Color) {
  // ...
}
```
- move_ant - obsługuje logikę przemieszczania się mrówki na planszy
```rust
fn move_ant(board: &mut Board, mut rng: &mut ThreadRng) {
  // ...
}
```
- setup - odczytuje zmienne środowiskowe lub parametry linii polecen w celu konfiguracji symulacji
```rust
fn setup() -> Config {
  // ...
}
```
