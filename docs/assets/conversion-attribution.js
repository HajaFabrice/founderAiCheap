(function () {
  const storageKey = "oplurix-attribution";
  const lastClickKey = "oplurix-last-conversion-click";

  function slugify(text) {
    return (text || "")
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/^-+|-+$/g, "")
      .slice(0, 80);
  }

  function pageSlug() {
    const path = window.location.pathname || "";
    const parts = path.split("/").filter(Boolean);
    const file = parts[parts.length - 1] || "index.html";
    return slugify(file.replace(/\.html$/i, "")) || "homepage";
  }

  function inferSource(campaign) {
    const value = (campaign || "").toLowerCase();
    if (value.includes("linkedin")) return "linkedin";
    if (value.includes("facebook")) return "facebook";
    if (value.includes("whatsapp")) return "whatsapp";
    if (value.includes("email")) return "email";
    if (value.includes("youtube")) return "youtube";
    return "oplurix-site";
  }

  function readStoredAttribution() {
    try {
      return JSON.parse(window.sessionStorage.getItem(storageKey) || "null") || {};
    } catch (_error) {
      return {};
    }
  }

  function writeStoredAttribution(context) {
    try {
      window.sessionStorage.setItem(storageKey, JSON.stringify(context));
    } catch (_error) {
      // Attribution is useful, not critical.
    }
  }

  function currentAttribution() {
    const params = new URLSearchParams(window.location.search);
    const stored = readStoredAttribution();
    const campaign = params.get("utm_campaign") || params.get("campaign") || stored.utm_campaign || "direct-site";
    const context = {
      utm_source: params.get("utm_source") || stored.utm_source || inferSource(campaign),
      utm_medium: params.get("utm_medium") || params.get("surface") || stored.utm_medium || "site",
      utm_campaign: campaign,
      utm_term: params.get("utm_term") || params.get("interest") || params.get("lang") || stored.utm_term || ""
    };

    const hasInboundAttribution = [
      "utm_source",
      "utm_medium",
      "utm_campaign",
      "utm_term",
      "campaign",
      "surface",
      "interest",
      "lang"
    ].some((key) => params.has(key));

    if (hasInboundAttribution) {
      writeStoredAttribution(context);
    }

    return context;
  }

  function platformFor(url) {
    const host = url.hostname.toLowerCase();
    if (host === "payhip.com" || host.endsWith(".payhip.com")) return "payhip";
    if (host === "shopify.com" || host.endsWith(".shopify.com") || host.endsWith(".myshopify.com")) return "shopify";
    return "";
  }

  function linkContent(anchor) {
    return (
      anchor.dataset.utmContent ||
      anchor.dataset.ctaSurface ||
      anchor.dataset.conversionSurface ||
      slugify(anchor.textContent) ||
      "checkout-link"
    );
  }

  function setIfMissing(params, key, value) {
    if (value && !params.has(key)) {
      params.set(key, value);
    }
  }

  function decorateCheckoutLink(anchor, attribution) {
    let url;
    try {
      url = new URL(anchor.href, window.location.href);
    } catch (_error) {
      return;
    }

    const platform = platformFor(url);
    if (!platform) {
      return;
    }

    const content = `${pageSlug()}__${linkContent(anchor)}`;
    setIfMissing(url.searchParams, "utm_source", attribution.utm_source);
    setIfMissing(url.searchParams, "utm_medium", attribution.utm_medium);
    setIfMissing(url.searchParams, "utm_campaign", attribution.utm_campaign);
    setIfMissing(url.searchParams, "utm_content", content);
    setIfMissing(url.searchParams, "utm_term", attribution.utm_term);

    anchor.href = url.toString();
    anchor.dataset.conversionPlatform = platform;
    anchor.dataset.conversionEvent = platform === "payhip" ? "payhip_checkout_click" : "shopify_checkout_click";
    anchor.dataset.utmContent = content;

    if (anchor.dataset.conversionBound === "true") {
      return;
    }

    anchor.addEventListener("click", function () {
      const event = {
        event: anchor.dataset.conversionEvent,
        platform,
        href: anchor.href,
        page: pageSlug(),
        utm_source: url.searchParams.get("utm_source") || "",
        utm_medium: url.searchParams.get("utm_medium") || "",
        utm_campaign: url.searchParams.get("utm_campaign") || "",
        utm_content: url.searchParams.get("utm_content") || "",
        utm_term: url.searchParams.get("utm_term") || ""
      };

      window.dataLayer = window.dataLayer || [];
      window.dataLayer.push(event);

      try {
        window.sessionStorage.setItem(lastClickKey, JSON.stringify(event));
      } catch (_error) {
        // Best-effort only.
      }
    });

    anchor.dataset.conversionBound = "true";
  }

  function decorateCheckoutLinks() {
    const attribution = currentAttribution();
    document.querySelectorAll("a[href]").forEach((anchor) => {
      decorateCheckoutLink(anchor, attribution);
    });
  }

  document.addEventListener("DOMContentLoaded", decorateCheckoutLinks);
})();
