import React, { useState } from 'react';
import axios from 'axios';

const apiUrl = "http://141.148.211.138:8080";

const URLShortener = () => {
  const [longUrl, setLongUrl] = useState('');
  const [shortUrl, setShortUrl] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = async (e) => {
    e.preventDefault();
    setIsLoading(true);
    setError('');

    try {
      const response = await axios.post(`${apiUrl}/url`, {
        target_url: longUrl,
      });
      setShortUrl(response.data.id);
    } catch (err) {
      console.error('Error:', err);
      setError('An error occurred while shortening the URL.');
    } finally {
      setIsLoading(false);
    }
  };

  const copyToClipboard = () => {
    navigator.clipboard.writeText(`${apiUrl}/url/${shortUrl}`)
      .then(() => alert('Copied to clipboard!'))
      .catch(() => alert('Failed to copy. Please try manually.'));
  };

  return (
    <div className="container">
      <div className="card">
        <div className="card-header">
          <h1>URL Shortener</h1>
          <p>Simplify your links in seconds</p>
        </div>
        <div className="card-content">
          <form onSubmit={handleSubmit} className="form">
            <div className="input-group">
              <label htmlFor="longUrl">Long URL</label>
              <input
                id="longUrl"
                type="url"
                placeholder="https://example.com/very/long/url"
                value={longUrl}
                onChange={(e) => setLongUrl(e.target.value)}
                required
              />
            </div>
            <button 
              type="submit" 
              className="submit-button"
              disabled={isLoading}
            >
              {isLoading ? 'Shortening...' : 'Shorten URL'}
            </button>
          </form>

          {error && <p className="error">{error}</p>}

          {shortUrl && (
            <div className="result">
              <label htmlFor="shortUrl">Shortened URL</label>
              <div className="flex">
                <input
                  id="shortUrl"
                  type="url"
                  value={`${apiUrl}/${shortUrl}`}
                  readOnly
                />
                <button onClick={copyToClipboard} className="copy-button">
                  Copy
                </button>
              </div>
              <a
                href={`${apiUrl}/${shortUrl}`}
                target="_blank"
                rel="noopener noreferrer"
                className="link"
              >
                Open shortened URL
              </a>
            </div>
          )}
        </div>
      </div>

      <footer className="footer">
        <a
          href="https://github.com/gurshaan17/blazingly-fast"
          target="_blank"
          rel="noopener noreferrer"
        >
          GitHub
        </a>
      </footer>
    </div>
  );
};

export default URLShortener;
