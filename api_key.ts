let apiKeyCache: { key: string; timestamp: number } | null = null;

export async function getApiKey(
  mode: "auto" | "random" = "auto"
): Promise<string> {
  const now = Date.now();

  if (apiKeyCache && now - apiKeyCache.timestamp < 500) {
    return apiKeyCache.key;
  }

  try {
    const response = await fetch(
      `http://localhost:8080/next?mode=${mode}`
    );
    if (!response.ok) {
      throw new Error("Failed to fetch API key");
    }
    const data = await response.json();
    const apiKey = data.api_key;

    apiKeyCache = { key: apiKey, timestamp: now };

    return apiKey;
  } catch (error) {
    console.error("Error fetching API key:", error);
    // Return a default or fallback key if needed
    return "default-fallback-key";
  }
}
