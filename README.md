# Самостійна робота з алгоритмів та структур даних

## Як тестувати:

1. Інсталюємо `Rust`: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. Клонуємо репозиторій:
    ```bash
    git clone https://github.com/varex83/sr_algo.git
    ```

3. Переходимо в папку з проектом:
    ```bash
    cd sr_algo
    ```

4. Запускаємо тести:
    ```bash
    cargo test
    ```

5. Запускаємо бінарні файли:

    - Стек:
        ```bash
        cargo run --bin stack
        ```

    - Множини:
        ```bash
        cargo run --bin unions
        ```

    - Черга:
        ```bash
        cargo run --bin queue
        ```

    - Бінарне Дерево Пошуку (BST):
        ```bash
        cargo run --bin bst
        ```

    - Перевірка дужок:
        ```bash
        cargo run --bin parentheses
        ```