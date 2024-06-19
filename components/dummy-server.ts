import express, { NextFunction, Request, Response } from 'express';

const app = express();

const htmlHeader = function (req: Request, res: Response, next: NextFunction) {
  res.html = (markup: string) => {
    res.setHeader("Content-Type", "text/html; charset=utf-8");
    res.send(markup);
  };
  next();
}

app.use(htmlHeader);

// Example middleware
app.get('/api/custom-endpoint', (req: Request, res: Response) => {
  res.html({ message: 'Hello from custom endpoint!' });
});

app.get('/hx-needs/:username', (req: Request, res: Response) => {
  res.html('Needs response');
});

app.get('/hx-notifs/:username', (req: Request, res: Response) => {
  res.html('Notifications response');
});

app.get('/hx-ledger/:username', (req: Request, res: Response) => {
  res.html('Ledger response');
});

export default app;
