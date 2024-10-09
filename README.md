
# Emscripten

## REALIZADO NO WINDOWS!

# Projeto Rust para WebAssembly

Este projeto demonstra como criar uma calculadora simples em Rust que utiliza WebAssembly para realizar soma. O exemplo inclui a instalação de ferramentas necessárias, a configuração do ambiente e a criação de um projeto básico.

## Instalação

### 1. Instalar Rust e Git

- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/downloads/win)

### 2. Adicionar a ferramenta de destino WebAssembly

Abra o git e execute:

```bash
rustup target add wasm32-unknown-emscripten
rustup target add wasm32-unknown-unknown # apenas para prevenir
```

### 3. Instalar Emscripten

Clone o repositório do Emscripten:

```bash
git clone https://github.com/emscripten-core/emsdk.git
```

#### 3.1 Acessar a pasta no git

```bash
cd emsdk
```

### 4. Instalar as ferramentas do SDK

Execute os seguintes comandos no git:

```bash
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh
```

### 5. Verificar se o `emcc` está funcionando

Execute no git:

```bash
emcc --version
```

### 6. Criar um novo projeto Rust

Na pasta `emsdk`, crie um novo projeto pelo git:

```bash
cargo new "nome_da_sua_pasta(sem aspas)" --lib
cd "nomedapasta(sem aspas)"
```
Por que --lib? Esta opção indica que o projeto será uma biblioteca, em vez de um executável. Quando você cria um projeto como biblioteca, o Cargo cria um arquivo lib.rs dentro do diretório src, onde você pode definir suas funções e estruturas que podem ser utilizadas por outros projetos ou pelo código JavaScript quando compilado para WebAssembly.

Navegue até a pasta do projeto (geralmente em `C:\Users\seu_usuario\emsdk\pasta_criada`) E ABRA NO VSCODE, vai gerar um `cargo.toml` e um `src`, onde vai ter o `lib.rs`, que é seu arquivo.

### 7. Escrever o código no `lib.rs`

No arquivo `lib.rs`, adicione o seguinte código para realizar a multiplicação:

```rust
// Importa o módulo prelude do wasm_bindgen, que contém as definições necessárias
// para usar as funcionalidades do WebAssembly no Rust.
use wasm_bindgen::prelude::*;

// O atributo #[wasm_bindgen] é necessário para que o compilador saiba que esta
// função deve ser exposta ao JavaScript quando o código Rust for compilado para
// WebAssembly. Isso permite que a função possa ser chamada a partir do código JavaScript.
#[wasm_bindgen]
// Define a função `add` que recebe dois parâmetros do tipo inteiro (i32).
// O nome da função será acessível a partir do JavaScript após a compilação.
pub fn add(a: i32, b: i32) -> i32 {
    // A função retorna o resultado da soma dos dois parâmetros.
    // O tipo de retorno é também um inteiro (i32), o que é consistente com os
    // tipos de entrada.
    a + b
}

```

### 8. Atualizar o `Cargo.toml`

No arquivo `Cargo.toml`, normalmente gerado na pasta criado pelo cargo new, adicione as seguintes linhas:

```toml
[package]
name = "rust_to_wasm"  # Nome do seu projeto
version = "0.1.0"      # Versão do projeto
edition = "2021"       # Edição do Rust

[dependencies]
wasm-bindgen = "0.2"   # Adicione essa dependência para utilizar o wasm-bindgen

[lib]
crate-type = ["cdylib"] # Isso é necessário para compilar para WebAssembly
```

### 9. Construir o projeto

Execute no git para instalação do wasm-pack:
```bash
cargo install wasm-pack
```
Na pasta do seu projeto, execute no git, onde aqui ele vai transformar o seu arquivo lib.rs em um pacote onde vai ter webassembly, javascript e outros.
```bash
wasm-pack build --target web
```
Pra que serve?, é um comando essencial para compilar projetos Rust para WebAssembly, preparando-os para serem utilizados em aplicações web. Ele automatiza o processo de compilação, geração de wrappers JavaScript, e estruturação do pacote, tornando a integração com projetos JavaScript simples e direta.


### Depois disso, será criado uma pasta pkg onde contém os arquivos WEBASSEMBLY `C:\Users\seu_usuario\emsdk\pasta_criada\acho q vai estar por aq kkkk`

# Você irá criar uma pasta em htdocs(SEU PROJETO)(XAMPP) e colocará os seguintes arquivos:
## Uma pasta apenas com o lib.rs, a pasta com o pkg gerado(webassembly) e um index criado para interligar o JavaScript gerado pelo WebAssembly. Depois só ligar o Apache e pequisar por localhost/nome_do_projeto e testar o código

### 10. Criar um arquivo HTML

Crie um arquivo HTML para interagir com o WebAssembly:

```html
<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Calculadora em Rust e WebAssembly</title>
    <link rel="stylesheet" href="index.css"> <!-- Link para o CSS -->
    <script type="module" src="index.js"></script> <!-- Importa o JavaScript -->
</head>
<body>
    <div class="container"> <!-- Adiciona uma div container para aplicar o CSS -->
        <h1>Calculadora em Rust e WebAssembly</h1>
        <label for="inputA">Digite o primeiro número:</label>
        <input type="number" id="inputA" placeholder="0" />

        <label for="inputB">Digite o segundo número:</label>
        <input type="number" id="inputB" placeholder="0" />

        <button id="calculateButton">Calcular Multiplicação</button>
        <div id="result"></div>
    </div>
</body>
</html>

```
# Arquivo JavaScript separado apenas por boas práticas (ESSE JAVASCRIPT FOI GERADO JUNTO COM O INDEX, FOI APENAS SEPARADO) ENQUANTO O JAVASCRIPT GERADO PELO WEBASSEMBLY FICA LÁ NA PASTA PKG, AQUI VOCÊ SÓ "CONECTA"
```javascript
import init, { add } from './pkg/calculadora.js'; // Altere para o nome do seu arquivo .js gerado

        async function run() {
            await init(); // Inicializa o módulo WebAssembly

            // Obtém os valores dos inputs
            const a = parseInt(document.getElementById("inputA").value);
            const b = parseInt(document.getElementById("inputB").value);

            // Verifica se os valores são números
            if (isNaN(a) || isNaN(b)) {
                document.getElementById("result").innerText = 'Por favor, insira números válidos.';
                return;
            }

            // Chama a função add do WebAssembly
            const result = add(a, b);
            // Exibe o resultado
            document.getElementById("result").innerText = `Resultado: ${result}`;
        }

        // Adiciona o Event Listener para o botão
        window.addEventListener('DOMContentLoaded', () => {
            document.getElementById("calculateButton").addEventListener('click', run);
        });
```
ISTO É UMA FORMA DE FAZER, existe também a de em vez de chamar o arquivo javascript, chamar o arquivo wasm.

JavaScript é melhor para:

Aplicações que dependem de muita interatividade com o DOM (manipulação de elementos HTML).
Projetos pequenos e médios, onde a simplicidade e a velocidade de desenvolvimento são essenciais.
Tarefas comuns da web, como criação de sites, manipulação de formulários e animações.
Back-end em Node.js ou outras APIs baseadas em JavaScript.

WebAssembly é melhor para:

Aplicações que precisam de alto desempenho, como jogos, editores de vídeo/imagem ou qualquer coisa que envolva algoritmos complexos.
Reutilizar código existente de linguagens como Rust, C ou C++.
Cenários onde o JavaScript não consegue entregar o desempenho desejado, como em cálculos intensivos ou tarefas com grandes volumes de dados.

### formato WASM, você só cria uma função para ativar o WASM (ALTERAÇÕES FEITAS NO INDEX.JS)
```javascript
// Função que carrega o WebAssembly e utiliza a função `add` exportada
async function loadWasm() {
    // Carrega o arquivo `.wasm`
    const response = await fetch('./pkg/calculadora_bg.wasm');
    const wasmModule = await WebAssembly.instantiateStreaming(response);
    return wasmModule.instance.exports;
}

async function run() {
    // Inicializa o WebAssembly
    const wasmInstance = await loadWasm();

    // Obtém os valores dos inputs
    const a = parseInt(document.getElementById("inputA").value);
    const b = parseInt(document.getElementById("inputB").value);

    // Verifica se os valores são números
    if (isNaN(a) || isNaN(b)) {
        document.getElementById("result").innerText = 'Por favor, insira números válidos.';
        return;
    }

    // Chama a função `add` do WebAssembly
    const result = wasmInstance.add(a, b);
    // Exibe o resultado
    document.getElementById("result").innerText = `Resultado: ${result}`;
}

// Adiciona o Event Listener para o botão
window.addEventListener('DOMContentLoaded', () => {
    document.getElementById("calculateButton").addEventListener('click', run);
});

```

## Executando o Projeto

SÓ COLOCAR UM CSS PRA FICAR BONITINHO E PARTIR PARA O ABRAÇO