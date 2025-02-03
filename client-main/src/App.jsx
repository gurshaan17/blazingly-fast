import React, { useState } from 'react';
import axios from 'axios';
import './App.css';

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
    <div>
    <div className="url-shortener-container">
      <div className="url-shortener-card">
        <div className="url-shortener-header">
          <h1>URL Shortener</h1>
          <p>Simplify your links in seconds</p>
        </div>
        
        <form onSubmit={handleSubmit} className="url-shortener-form">
          <div className="input-group">
            <label htmlFor="longUrl">Long URL</label>
            <input
              id="longUrl"
              type="url"
              placeholder="https://example.com/very/long/url"
              value={longUrl}
              onChange={(e) => setLongUrl(e.target.value)}
              // style={{background: "white"}}
              required
            />
          </div>
          
          <button
            type="submit"
            disabled={isLoading}
          >
            {isLoading ? 'Shortening...' : 'Shorten URL'}
          </button>
        </form>

        {error && <p className="error-message">{error}</p>}

        {shortUrl && (
          <div className="result-section">
            <label htmlFor="shortUrl">Shortened URL</label>
            <div className="short-url-container">
              <input
                id="shortUrl"
                type="url"
                value={`${apiUrl}/url/${shortUrl}`}
                readOnly
              />
              <button onClick={copyToClipboard}>Copy</button>
            </div>
            <a
              href={`${apiUrl}/url/${shortUrl}`}
              target="_blank"
              rel="noopener noreferrer"
            >
              Open shortened URL
            </a>
          </div>
        )}

        <footer className="url-shortener-footer">
          <a
            href="https://github.com/gurshaan17/blazingly-fast"
            target="_blank"
            rel="noopener noreferrer"
          >
            GitHub
          </a>
        </footer>
      </div>
    </div>
    </div>
  );
};

export default URLShortener;