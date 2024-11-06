import init, { analisar_senhas } from '../../pkg/password_analyzer.js';

// Inicializa o WebAssembly e a aplicação
async function run() {
    await init(); // Inicializa o WebAssembly

    document.getElementById('analyze').onclick = async () => {
        await processPasswordsFromTextarea();
    };

    document.getElementById('fileInput').addEventListener('change', async (event) => {
        const file = event.target.files[0]; // Obtém o arquivo selecionado pelo usuário
        if (file) {
            await processPasswordsFromFile(file);
        }
    }); 
}

// Função para processar senhas da área de texto
async function processPasswordsFromTextarea() {
    document.getElementById('loading').style.display = 'block'; // Mostra o carregando
    const passwords = document.getElementById('passwords').value.split('\n').map(s => s.trim()).filter(Boolean);
    const results = await analisar_senhas(passwords);
    displayResults(JSON.parse(results));
}

// Função para processar senhas de um arquivo
async function processPasswordsFromFile(file) {
    const reader = new FileReader(); // Cria um leitor de arquivos

    reader.onload = async (e) => {
        const content = e.target.result; // Lê o conteúdo do arquivo
        let passwords = [];

        if (file.type === 'application/json' || file.name.endsWith('.json')) {
            passwords = await parseJson(content);
        } else if (file.type === 'text/csv' || file.name.endsWith('.csv')) {
            passwords = parseCsv(content);
        } else {
            alert('Formato de arquivo não suportado. Use arquivos JSON ou CSV.');
            return;
        }


        const results = await analisar_senhas(passwords);
        displayResults(JSON.parse(results));


    };

    reader.readAsText(file); // Lê o arquivo como texto
}

// Função para exibir os resultados
function displayResults(results) {
    document.getElementById('results').innerText = results.join('\n'); // Exibe os resultados na página
    document.getElementById('loading').style.display = 'none'; // Esconde o carregando
}

// Função para analisar JSON (in this example, returns an array of passwords)
async function parseJson(content) {
  try {
      const jsonData = JSON.parse(content);
      if (!Array.isArray(jsonData.passwords)) {
          throw new Error('O arquivo JSON deve conter um objeto com uma propriedade "passwords" que é um array.');
      }
      return jsonData.passwords; // Returns the array of passwords from the JSON
  } catch (error) {
      alert('Erro ao analisar o arquivo JSON: ' + error.message);
      return []; // Return an empty array in case of an error to prevent further issues.
  }
}


// Função para analisar CSV
function parseCsv(content) {
    return content.split('\n').map(row => row.trim()).filter(row => row.length > 0); // Remove linhas vazias
}

// Chama a função principal ao carregar a página
run();