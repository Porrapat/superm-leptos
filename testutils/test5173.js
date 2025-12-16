import http from 'k6/http';
import { sleep } from 'k6';

export const options = {
  vus: 50,
  duration: '30s',
};

export default function () {
  http.get('http://localhost:5173', {
    tags: { name: 'frontend_5173' }
  });

  sleep(1);
}
