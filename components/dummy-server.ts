import express, { Request, Response } from 'express';

const app = express();

// Example middleware
app.use('/api/custom-endpoint', (req: Request, res: Response) => {
  res.json({ message: 'Hello from custom endpoint!' });
});

export default app;
