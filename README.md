# Symulator mrowiska

Program symuluje kolonie mrówek. Mrówki przemieszczają się w losowym kierunku oraz podnoszą i upuszczają przedmioty na planszy heksagonalnej.

Program wykorzystuje bibliotekę Raylib w celu wizualizacji mrowiska.

![image](https://github.com/bjaglowski/anthill/assets/89010497/0b3cf705-a620-4de8-97d2-db3db6627fa8)


## Uruchomienie
```shell
cargo run
# lub
cargo run [SZEROKOŚĆ PLANSZY] [WYSOKOŚĆ PLANSZY] [ILOŚĆ ITERACJI] [CZAS POMIĘDZY ITERACJAMI W MILISEKUNDACH]
```

## Zasady działania symulacji

- Plansza ma pewną ilość pól i nie posiada granic, tzn mrowisko jest skończone, lecz nieograniczone (mrówka przemieszczająca się do krawędzi okna znajdzie się po jego drugiej stronie).
- Przedmioty dzielą się na:
  - liście oznaczone kolorem niebieskim
  - patyki oznaczone kolorem żółtym
- Mrówka (oznaczona kolorem czerwonym) w każdej iteracji symulacji przemieszcza się pięciokrotnie w losowych kierunkach, w przypadku planszy heksagonalnej - mrówka może przemieszczać się w sześciu kierunkach.
- Mrówka, która nie przenosi przedmiotu może wejść na pole z przedmiotem, aby go podnieść.
- Mrówka, która przenosi przedmiot zmienia swój kolor na pomarańczowy.
- Mrówka, nie może wejść na pole z innym przedmiotem, jeśli w danym momencie przenosi ona przedmiot. Może jednak współdzielić pole z inną mrówką.
- W kolejnej iteracji, jeśli mrówka sąsiaduje z przedmiotem o tym samym typie. Przymocowuje przedmiot na polu na którym stoi.
- Przedmiot jest przymocowywany do pola z siłą proporcjonalną do ilości sąsiadujących w danym momencie przedmiotów o tym samym typie, dzięki czemu mrówki lepiej "współpracują" przy budowie struktur.

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
- draw_hexagon - rysuje heksagon na podanej pozycji, o podanym rozmiarze i kolorze
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
- setup - odczytuje zmienne środowiskowe lub parametry linii poleceń w celu konfiguracji symulacji
```rust
fn setup() -> Config {
  // ...
}
```

## Wybrane fragmenty implementacji
### Przymocowywanie nowych przedmiotów z "siłą"
Podczas implementacji okazało się, że mrówkom dosyć ciężko przebiega proces budowania większych struktur. Dlatego przed odłożeniem przedmiotu przez mrówkę na planszy - sprawdzana jest ilość sąsiadujących przedmiotów o identycznym typie. Ilość takich sąsiadów jest wykładnikiem potęgi przymocowania przedmiotu do planszy. Kolejne mrówki przed podniesieniem takiego przedmiotu muszą go "podgryzać", aby osłabić siłę ich przymocowania. Po wyzerowaniu wartości `freeze_time_left` przedmiot może zostać podniesiony i przemieszczony w inne miejsce.
```rust
                    if item.freeze_time_left == 0 {
                        // the item's attach is sufficiently weakened to pick
                        ant.carrying_item = Some(item);
                    } else {
                        // the item is attached too hard to pick, bite it
                        item.freeze_time_left = item.freeze_time_left - 1;
                        self.items.push(item);
                    }
```
```rust
                    if similar_items_nearby_count > 0 {

                        // attach the item harder if more elements there are nearby
                        let freeze_time_left = 2_u32.pow(similar_items_nearby_count as u32) as usize;
                        self.items.push(Item::new(ant.x, ant.y, freeze_time_left, carrying_item.item_type));
                        ant.carrying_item = None;
                        break
                    }
```
### Ruch mrówek
Mrówki podczas każdej iteracji poruszają się pięciokrotnie w losowych kierunkach, aby mogły trochę oddalić się od budowanej przez siebie struktury w celu poszukiwania kolejnych przedmiotów. Jednocześnie, jeśli przenoszą w danej chwili przedmiot - nie wchodzą na pola z innymi przedmiotami.
```rust
fn move_ant(board: &mut Board, mut rng: &mut ThreadRng) {
    for ant in &mut board.ants {
        // move ant by 5 fields randomly
        for _ in 0..5 {
            let mut neighbors = ant.get_neighbors(board.max_x, board.max_y).into_iter().collect::<Vec<_>>();
            neighbors.shuffle(&mut rng);


            for (nx, ny) in neighbors {
                // block move if ant which is carrying an item is trying to step on another item
                if let Some(_) = &ant.carrying_item {
                    if board.items.iter().any(|item| item.x == nx && item.y == ny) {
                        continue;
                    }
                }
                ant.x = nx;
                ant.y = ny;
                break;
            }
        }
    }
}
```
### Obliczanie pozycji sąsiadujących pól
Logicznie plansza mogłaby być też przedstawiona w postaci siatki złożonej z kwadratów. Przedstawienie planszy w postaci heksagonalnej jest możliwe dzięki przesunięciu pól w wierszach nieparzystych o pół szerokości pojedynczego pola.
```rust
  fn calculate_center_x(&self, x: usize, y: usize, radius: f32) -> f32 {
      // https://www.redblobgames.com/grids/hexagons/
      x as f32 * 3.0_f32.sqrt() * radius + if y % 2 == 0 { radius } else { radius + 3.0_f32.sqrt() * radius / 2.0 }
  }
```
Dodatkowo każde pole sąsiaduje z sześcioma innymi, dzięki dodaniu nowych kierunków ruchu. Z faktu, że co drugi z wierszy na planszy jest "przesunięty" w prawo, dla obu przypadków implementacja dozwolonych ruchów wygląda inaczej.
```rust
fn get_neighbors(&self, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    // get neighboring fields
    let x: i32 = self.x as i32;
    let y: i32 = self.y as i32;
    
    
    if self.y % 2 == 0 {
        vec![
            ((x - 1).rem_euclid(max_x as i32) as usize, (y - 1).rem_euclid(max_y as i32) as usize),
            (self.x, (y - 1).rem_euclid(max_y as i32) as usize),
            ((self.x + 1).rem_euclid(max_x), self.y),
            (self.x, (self.y + 1).rem_euclid(max_y)),
            ((x - 1).rem_euclid(max_x as i32) as usize, (self.y + 1).rem_euclid(max_y)),
            ((x - 1).rem_euclid(max_x as i32) as usize, self.y),
        ]
    } else {
        vec![
            (self.x, (self.y - 1).rem_euclid(max_y)),
            ((self.x + 1).rem_euclid(max_x), (y - 1).rem_euclid(max_y as i32) as usize),
            ((self.x + 1).rem_euclid(max_x), self.y),
            ((self.x + 1).rem_euclid(max_x), (self.y + 1).rem_euclid(max_y)),
            (self.x, (self.y + 1).rem_euclid(max_y)),
            ((x - 1).rem_euclid(max_x as i32) as usize, self.y),
        ]
    }
}
```
### Renderowanie heksagonów
Do tego celu została użyta biblioteka Raylib, a dokładniej funkcja 'draw_poly'
```rust
fn draw_hexagon(d: &mut RaylibDrawHandle, x: f32, y: f32, radius: f32, color: Color) {
    d.draw_poly(Vector2::new(x, y), 6, radius, 0.0, color);
}
```

