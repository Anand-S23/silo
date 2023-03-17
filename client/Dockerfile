FROM node:18-alpine

WORKDIR /usr/src/client
COPY package.json ./
RUN npm install

COPY . .

CMD ["npm", "run", "dev"]