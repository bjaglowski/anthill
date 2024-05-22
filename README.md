# Symulator mrowiska

Program symuluje kolonie mrówek. Mrówki przemieszczają się w losowym kierunku oraz podnoszą i upuszczają przedmioty na planszy heksagonalnej.

Program wykorzystuje bibliotekę Raylib w celu wizualizacji mrowiska.

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
