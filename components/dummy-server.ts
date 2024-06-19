import express, { Request, Response } from 'express';

const app = express();

// Example middleware
app.get('/api/custom-endpoint', (req: Request, res: Response) => {
  res.json({ message: 'Hello from custom endpoint!' });
});

app.get('/hx-needs/:username', (req: Request, res: Response) => {
  res.send('Needs response')
});

app.get('/hx-notifs/:username', (req: Request, res: Response) => {
  res.send('Notifications response')
});

app.get('/hx-ledger/:username', (req: Request, res: Response) => {
  res.send('Ledger response')
});

export default app;
