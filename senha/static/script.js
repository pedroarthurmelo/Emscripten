import init, { analisar_senhas } from '../../pkg/password_analyzer.js'; // Importa o módulo WebAssembly (WASM) e a função de análise de senhas

// Função principal que inicializa o WebAssembly e configura os eventos da página
async function run() {
    await init(); // Inicializa o módulo WASM para garantir que o WebAssembly esteja pronto

    // Configura o evento de clique no botão de análise de senhas
    document.getElementById('analyze').onclick = async () => {
        await processPasswordsFromTextarea(); // Processa as senhas a partir da área de texto
    };

    // Configura o evento de mudança para o input de arquivo
    document.getElementById('fileInput').addEventListener('change', async (event) => {
        const file = event.target.files[0]; // Obtém o arquivo selecionado pelo usuário
        if (file) {
            await processPasswordsFromFile(file); // Processa as senhas a partir do arquivo
        }
    }); 
}

// Função para processar senhas da área de texto
async function processPasswordsFromTextarea() {
    document.getElementById('loading').style.display = 'block'; // Exibe a mensagem de carregamento
    const passwords = document.getElementById('passwords').value.split('\n').map(s => s.trim()).filter(Boolean); 
    // Obtém as senhas da área de texto, divide em linhas, remove espaços extras e filtra entradas vazias.
    
    const results = await analisar_senhas(passwords); // Chama a função WASM para analisar as senhas
    displayResults(JSON.parse(results)); // Exibe os resultados na página
}

// Função para processar senhas de um arquivo
async function processPasswordsFromFile(file) {
    const reader = new FileReader(); // Cria um leitor de arquivos para ler o conteúdo do arquivo

    reader.onload = async (e) => {
        const content = e.target.result; // Obtém o conteúdo do arquivo após a leitura
        let passwords = []; // Inicializa um array para armazenar as senhas

        // Verifica o tipo de arquivo e chama a função apropriada para analisar as senhas
        if (file.type === 'application/json' || file.name.endsWith('.json')) {
            passwords = await parseJson(content); // Se for um arquivo JSON, parseia o conteúdo
        } else if (file.type === 'text/csv' || file.name.endsWith('.csv')) {
            passwords = parseCsv(content); // Se for um arquivo CSV, parseia o conteúdo
        } else {
            alert('Formato de arquivo não suportado. Use arquivos JSON ou CSV.'); // Alerta se o formato não for suportado
            return;
        }

        const results = await analisar_senhas(passwords); // Analisa as senhas utilizando WebAssembly
        displayResults(JSON.parse(results)); // Exibe os resultados
    };

    reader.readAsText(file); // Lê o arquivo como texto
}

// Função para exibir os resultados na página
function displayResults(results) {
    document.getElementById('results').innerText = results.join('\n'); // Exibe cada resultado na tela, separando por novas linhas
    document.getElementById('loading').style.display = 'none'; // Esconde a mensagem de carregamento
}

// Função para analisar um arquivo JSON
async function parseJson(content) {
  try {
      const jsonData = JSON.parse(content); // Tenta fazer o parse do conteúdo como JSON
      if (!Array.isArray(jsonData.passwords)) { // Verifica se o JSON contém uma propriedade "passwords" que seja um array
          throw new Error('O arquivo JSON deve conter um objeto com uma propriedade "passwords" que é um array.');
      }
      return jsonData.passwords; // Retorna o array de senhas do JSON
  } catch (error) {
      alert('Erro ao analisar o arquivo JSON: ' + error.message); // Exibe um erro se o JSON não for válido
      return []; // Retorna um array vazio para evitar que o processamento continue com dados inválidos
  }
}

// Função para analisar um arquivo CSV
function parseCsv(content) {
    return content.split('\n').map(row => row.trim()).filter(row => row.length > 0); 
    // Divide o conteúdo do CSV por linhas, remove espaços extras e filtra linhas vazias
}

// Chama a função principal ao carregar a página
run(); // Inicia a execução quando a página for carregada
