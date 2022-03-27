const fs = require('fs')

const data = fs.readFileSync('./words_alpha.txt', 'utf8');

const words = {};

// Convert character to offset from lowercase 'a'
function char_to_int(char) {
  return char.charCodeAt(0) - 97;
}

const IS_WORD_KEY = 26;

data.split(/\r?\n/).forEach(word => {
  let cursor = words;
  for (const c of word) {
    const index = char_to_int(c);
    cursor[index] = cursor[index] || {};
    cursor = cursor[index];
  }
  cursor[IS_WORD_KEY] = true;
});
const used = process.memoryUsage().heapUsed / 1024 / 1024;
console.log(`The script uses approximately ${Math.round(used * 100) / 100} MB`);