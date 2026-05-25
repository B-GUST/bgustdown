#!/usr/bin/env node
const { Bgustdown } = require('./index.js');
const fs = require('fs');
const path = require('path');

const [, , command, ...args] = process.argv;
const client = new Bgustdown();

async function main() {
  if (command === 'convert') {
    const filePath = path.resolve(args[0]);
    if (!fs.existsSync(filePath)) {
      console.error(`Error: File not found at ${filePath}`);
      process.exit(1);
    }
    const md = await client.convert(filePath);
    console.log(md);
  } else if (command === 'prepare') {
    const filePath = path.resolve(args[0]);
    const md = await client.convert(filePath);
    const sentences = client.prepareTrainingData(md, 'cli_source', 'cli_domain');
    console.log(JSON.stringify(sentences, null, 2));
  } else {
    console.log('Usage: bgustdown [convert|prepare] <file_path>');
  }
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
