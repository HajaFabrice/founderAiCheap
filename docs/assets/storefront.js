(function () {
  const store = {
    // Public PayPal client ID only. Never place the PayPal secret in this file.
    paypalClientId: "ASH8aFKSdfDYkIu8x5Aajj2kwKYtdGV355m54J0r-__TwCF1JALW7NP1CYlx3-r_LD-8L6WcAVCmezSO",
    currencyCode: "USD",
    sellerEmail: "hajafabriceeris@gmail.com",
    sellerPhone: "+261349414112",
    standardDeliveryWindowHours: 12,
    products: {
      "content-engine": {
        id: "content-engine",
        nameEn: "Expert-to-Influencer Content Engine",
        nameFr: "Expert-to-Influencer Content Engine",
        priceUsd: 39,
        checkoutEnabled: true,
        payhipUrl: "https://payhip.com/buy?s=1&cart_links%5B%5D=6I1p4&qty%5B6I1p4%5D=1"
      },
      "ecor-starter": {
        id: "ecor-starter",
        nameEn: "EcoR Toolkit Starter",
        nameFr: "EcoR Toolkit Starter",
        priceUsd: 29,
        checkoutEnabled: false
      },
      "ecor-complete": {
        id: "ecor-complete",
        nameEn: "EcoR Toolkit Complete",
        nameFr: "EcoR Toolkit Complete",
        priceUsd: 97,
        checkoutEnabled: false
      },
      "ecor-pro": {
        id: "ecor-pro",
        nameEn: "EcoR Toolkit Pro Bundle",
        nameFr: "EcoR Toolkit Pro Bundle",
        priceUsd: 297,
        checkoutEnabled: false
      },
      "ecor-institutional": {
        id: "ecor-institutional",
        nameEn: "EcoR Toolkit Institutional",
        nameFr: "EcoR Toolkit Institutional",
        priceUsd: 499,
        checkoutEnabled: false
      },
      "training-to-quiz-generator": {
        id: "training-to-quiz-generator",
        nameEn: "Training-to-Quiz Generator",
        nameFr: "Training-to-Quiz Generator",
        priceUsd: 29,
        checkoutEnabled: false
      },
      "gear-equipment-concierge": {
        id: "gear-equipment-concierge",
        nameEn: "Gear & Equipment Concierge",
        nameFr: "Gear & Equipment Concierge",
        priceUsd: 39,
        checkoutEnabled: false
      },
      "biodiversity-pitch-deck-builder": {
        id: "biodiversity-pitch-deck-builder",
        nameEn: "Biodiversity Pitch Deck Builder",
        nameFr: "Biodiversity Pitch Deck Builder",
        priceUsd: 69,
        checkoutEnabled: false
      }
    }
  };

  const thankYouStorageKey = "oplurix-thankyou-context";

  function getLanguage() {
    const saved = window.localStorage.getItem("oplurix-lang");
    if (saved === "fr" || saved === "en") {
      return saved;
    }
    return (navigator.language || "").toLowerCase().startsWith("fr") ? "fr" : "en";
  }

  function setLanguage(lang) {
    document.body.dataset.lang = lang;
    document.documentElement.lang = lang;
    window.localStorage.setItem("oplurix-lang", lang);
    document.querySelectorAll("[data-set-lang]").forEach((button) => {
      button.classList.toggle("is-active", button.dataset.setLang === lang);
    });
  }

  function currentPageName() {
    const path = window.location.pathname || "";
    const bits = path.split("/");
    return bits[bits.length - 1] || "index.html";
  }

  function readTrackingParams() {
    const params = new URLSearchParams(window.location.search);
    return {
      campaignName: params.get("campaign") || params.get("campaign_name") || "",
      ctaSurface: params.get("surface") || params.get("cta_surface") || "",
      language: params.get("lang") || params.get("language") || "",
      interestType: params.get("interest") || params.get("interest_type") || ""
    };
  }

  function upsertHiddenInput(form, name, value) {
    let input = form.querySelector(`[name="${name}"]`);
    if (!input) {
      input = document.createElement("input");
      input.type = "hidden";
      input.name = name;
      form.appendChild(input);
    }
    input.value = value;
  }

  function hydrateTrackedForm(form) {
    const params = readTrackingParams();
    const lang = document.body.dataset.lang || getLanguage();
    const interestType = params.interestType || form.dataset.interestType || form.querySelector('[name="interest_type"]')?.value || "general";
    const ctaSurface = params.ctaSurface || form.dataset.ctaSurface || form.querySelector('[name="cta_surface"]')?.value || "onsite-form";
    const campaignName = params.campaignName || form.querySelector('[name="campaign_name"]')?.value || "direct-site";
    const language = params.language || lang;

    upsertHiddenInput(form, "language", language);
    upsertHiddenInput(form, "campaign_name", campaignName);
    upsertHiddenInput(form, "cta_surface", ctaSurface);
    upsertHiddenInput(form, "interest_type", interestType);
  }

  function hydrateTrackedForms() {
    document.querySelectorAll("form[data-netlify='true']").forEach((form) => {
      hydrateTrackedForm(form);
    });
  }

  function buildFormContext(form) {
    const data = new FormData(form);
    return {
      formName: data.get("form-name") || form.getAttribute("name") || "",
      sourcePage: data.get("source_page") || currentPageName(),
      language: data.get("language") || document.body.dataset.lang || getLanguage(),
      campaignName: data.get("campaign_name") || "direct-site",
      ctaSurface: data.get("cta_surface") || "onsite-form",
      interestType: data.get("interest_type") || form.dataset.interestType || "general",
      resourceName: data.get("resource_name") || "",
      updateTrack: data.get("update_track") || "",
      productInterest: data.get("product_interest") || "",
      name: data.get("name") || ""
    };
  }

  function setupTrackedForms() {
    document.querySelectorAll("form[data-netlify='true']").forEach((form) => {
      hydrateTrackedForm(form);
      if (form.dataset.trackingBound === "true") {
        return;
      }
      form.addEventListener(
        "submit",
        function () {
          hydrateTrackedForm(form);
          try {
            window.localStorage.setItem(thankYouStorageKey, JSON.stringify(buildFormContext(form)));
          } catch (_error) {
            // best-effort only
          }
        },
        { capture: true }
      );
      form.dataset.trackingBound = "true";
    });
  }

  function localizeNodes(selector, textEn, textFr) {
    document.querySelectorAll(selector).forEach((node) => {
      node.textContent = node.hasAttribute("data-fr") ? textFr : textEn;
    });
  }

  function updateThankYouButtons(config) {
    document.querySelectorAll("[data-thankyou-primary]").forEach((node) => {
      const isFr = node.hasAttribute("data-fr");
      node.href = isFr ? (config.primary.hrefFr || config.primary.href) : (config.primary.hrefEn || config.primary.href);
      node.textContent = isFr ? config.primary.fr : config.primary.en;
    });
    document.querySelectorAll("[data-thankyou-secondary]").forEach((node) => {
      const isFr = node.hasAttribute("data-fr");
      node.href = isFr ? (config.secondary.hrefFr || config.secondary.href) : (config.secondary.hrefEn || config.secondary.href);
      node.textContent = isFr ? config.secondary.fr : config.secondary.en;
    });
    document.querySelectorAll("[data-thankyou-tertiary]").forEach((node) => {
      const isFr = node.hasAttribute("data-fr");
      node.href = isFr ? (config.tertiary.hrefFr || config.tertiary.href) : (config.tertiary.hrefEn || config.tertiary.href);
      node.textContent = isFr ? config.tertiary.fr : config.tertiary.en;
    });
  }

  function loadPayPalSdk() {
    if (!store.paypalClientId) {
      return Promise.resolve(false);
    }

    if (window.paypal && window.paypal.Buttons) {
      return Promise.resolve(true);
    }

    if (window.__oplurixPayPalPromise) {
      return window.__oplurixPayPalPromise;
    }

    const script = document.createElement("script");
    script.src =
      "https://www.paypal.com/sdk/js?client-id=" +
      encodeURIComponent(store.paypalClientId) +
      "&currency=" +
      encodeURIComponent(store.currencyCode) +
      "&intent=capture&components=buttons";
    script.async = true;

    window.__oplurixPayPalPromise = new Promise((resolve) => {
      script.onload = function () {
        resolve(Boolean(window.paypal && window.paypal.Buttons));
      };
      script.onerror = function () {
        resolve(false);
      };
    });

    document.head.appendChild(script);
    return window.__oplurixPayPalPromise;
  }

  function fallbackMailto(product, lang) {
    const subject =
      lang === "fr"
        ? `Demande de checkout PayPal - ${product.nameFr}`
        : `PayPal checkout request - ${product.nameEn}`;
    const body =
      lang === "fr"
        ? `Bonjour,%0D%0A%0D%0AJe souhaite acheter ${product.nameFr} au prix de $${product.priceUsd}. Merci de me confirmer le paiement et la livraison.%0D%0A%0D%0AMerci.`
        : `Hello,%0D%0A%0D%0AI would like to buy ${product.nameEn} for $${product.priceUsd}. Please confirm the payment and delivery steps.%0D%0A%0D%0AThank you.`;
    return `mailto:${store.sellerEmail}?subject=${encodeURIComponent(subject)}&body=${body}`;
  }

  function paymentNoteText(lang, activeCheckout, productId) {
    const deliveryWindow = store.standardDeliveryWindowHours;

    if (productId === "ecor-pro") {
      if (lang === "fr") {
        return activeCheckout
          ? `Paiement PayPal actif. La livraison manuelle inclut le bundle Pro, les dates de support et l'invitation WhatsApp dans les ${deliveryWindow} heures apres confirmation du paiement.`
          : `Paiement PayPal securise quand disponible. La livraison manuelle du bundle Pro, des dates de support et de l'invitation WhatsApp suit dans les ${deliveryWindow} heures apres confirmation du paiement.`;
      }

      return activeCheckout
        ? `PayPal checkout is active. Manual delivery includes the Pro bundle, support-window dates, and the WhatsApp invite within ${deliveryWindow} hours after payment confirmation.`
        : `Secure PayPal checkout when available. Manual delivery of the Pro bundle, support-window dates, and the WhatsApp invite follows within ${deliveryWindow} hours after payment confirmation.`;
    }

    if (lang === "fr") {
      return activeCheckout
        ? `Paiement PayPal actif. La livraison manuelle suit par email dans les ${deliveryWindow} heures apres confirmation du paiement.`
        : `Paiement PayPal securise quand disponible. La livraison manuelle suit par email dans les ${deliveryWindow} heures apres confirmation du paiement.`;
    }

    return activeCheckout
      ? `PayPal checkout is active. Manual delivery follows by email within ${deliveryWindow} hours after payment confirmation.`
      : `Secure PayPal checkout when available. Manual delivery follows by email within ${deliveryWindow} hours after payment confirmation.`;
  }

  function wireFallbackLinks() {
    const lang = document.body.dataset.lang || getLanguage();
    document.querySelectorAll(".paypal-button").forEach((button) => {
      const product = store.products[button.dataset.paypalProduct];
      if (!product || product.payhipUrl || !product.checkoutEnabled) {
        return;
      }

      const note = document.querySelector(`[data-product-note="${button.dataset.paypalProduct}"]`);
      button.href = fallbackMailto(product, lang);
      button.removeAttribute("target");
      button.removeAttribute("rel");
      button.textContent =
        lang === "fr" ? "Probleme avec PayPal ? Demander un checkout manuel" : "PayPal issue? Request manual checkout";

      if (note) {
        note.textContent = paymentNoteText(lang, false, product.id);
      }
    });
  }

  function renderPayPalButtons() {
    if (!window.paypal || !window.paypal.Buttons) {
      return;
    }

    const lang = document.body.dataset.lang || getLanguage();
    document.querySelectorAll(".paypal-button").forEach((button) => {
      const product = store.products[button.dataset.paypalProduct];
      if (!product || product.payhipUrl || !product.checkoutEnabled) {
        return;
      }

      let target = button.previousElementSibling;
      if (!target || !target.classList.contains("paypal-render-target")) {
        target = document.createElement("div");
        target.className = "paypal-render-target";
        button.parentNode.insertBefore(target, button);
      }

      if (target.dataset.rendered === "true") {
        return;
      }

      const note = document.querySelector(`[data-product-note="${button.dataset.paypalProduct}"]`);

      window.paypal
        .Buttons({
          style: {
            layout: "vertical",
            color: "gold",
            shape: "pill",
            label: "paypal",
            tagline: false
          },
          createOrder: function (_data, actions) {
            return actions.order.create({
              purchase_units: [
                {
                  description: lang === "fr" ? product.nameFr : product.nameEn,
                  custom_id: product.id,
                  amount: {
                    currency_code: store.currencyCode,
                    value: product.priceUsd.toFixed(2)
                  }
                }
              ]
            });
          },
          onApprove: function (data, actions) {
            return actions.order.capture().then(function () {
              const label = lang === "fr" ? product.nameFr : product.nameEn;
              window.location.href =
                "thank-you.html?product=" +
                encodeURIComponent(label) +
                "&order=" +
                encodeURIComponent(data.orderID || "");
            });
          },
          onError: function () {
            if (note) {
              note.textContent =
                lang === "fr"
                  ? "Le checkout PayPal a rencontre un probleme. Utilisez le bouton de checkout manuel juste en dessous."
                  : "PayPal checkout hit a problem. Use the manual checkout button just below.";
            }
          }
        })
        .render(target)
        .then(function () {
          target.dataset.rendered = "true";
          if (note) {
            note.textContent = paymentNoteText(lang, true, product.id);
          }
        });
    });
  }

  function updateThankYouPage() {
    if (!document.querySelector("[data-page='thank-you']")) {
      return;
    }

    const params = new URLSearchParams(window.location.search);
    const product = params.get("product");
    const order = params.get("order");
    const hasPayment = Boolean(product);

    document.querySelectorAll("[data-thankyou-form-copy]").forEach((node) => {
      node.hidden = hasPayment;
    });
    document.querySelectorAll("[data-thankyou-payment-copy]").forEach((node) => {
      node.hidden = !hasPayment;
    });

    if (hasPayment) {
      document.querySelectorAll("[data-thankyou-product]").forEach((node) => {
        node.textContent = product;
      });
      document.querySelectorAll("[data-thankyou-form-context]").forEach((node) => {
        node.hidden = true;
      });
    }

    const orderBlocks = document.querySelectorAll("[data-thankyou-order-copy]");
    if (order) {
      orderBlocks.forEach((node) => {
        node.hidden = false;
      });
      document.querySelectorAll("[data-thankyou-order]").forEach((node) => {
        node.textContent = order;
      });
    } else {
      orderBlocks.forEach((node) => {
        node.hidden = true;
      });
    }

    if (hasPayment) {
      return;
    }

    let storedContext = null;
    const contextFromQuery = {
      formName: params.get("form") || "",
      sourcePage: params.get("source") || "",
      language: params.get("lang") || params.get("language") || "",
      campaignName: params.get("campaign") || "",
      ctaSurface: params.get("surface") || "",
      interestType: params.get("interest") || "",
      resourceName: params.get("resource") || "",
      updateTrack: params.get("track") || "",
      productInterest: params.get("topic") || ""
    };
    try {
      storedContext = JSON.parse(window.localStorage.getItem(thankYouStorageKey) || "null");
      window.localStorage.removeItem(thankYouStorageKey);
    } catch (_error) {
      storedContext = null;
    }

    const context = storedContext || contextFromQuery;
    const variant =
      context.interestType === "checklist" || context.formName === "ethics-checklist-interest"
        ? "checklist"
        : context.interestType === "contact" || context.formName === "oplurix-product-interest"
          ? "contact"
          : context.interestType === "updates" || context.formName === "oplurix-updates"
            ? "updates"
            : "default";

    const contextMap = {
      checklist: {
        bodyEn: "Your checklist signup has been recorded. The printable version is ready now below, and this same path can later move into automatic email delivery once Kit is connected.",
        bodyFr: "Votre inscription a la checklist a bien ete enregistree. La version imprimable est deja prete ci-dessous, et ce meme chemin pourra ensuite passer en livraison email automatique une fois Kit connecte.",
        detailEn: context.campaignName && context.campaignName !== "direct-site"
          ? `Campaign source recorded: ${context.campaignName}.`
          : "Current track: Printable ethics checklist.",
        detailFr: context.campaignName && context.campaignName !== "direct-site"
          ? `Source de campagne enregistree : ${context.campaignName}.`
          : "Suivi actuel : checklist ethique imprimable.",
        primary: { hrefEn: "checklists/ethics-checklist-en.html", hrefFr: "checklists/ethics-checklist-fr.html", en: "Open the printable checklist", fr: "Ouvrir la checklist imprimable" },
        secondary: { hrefEn: "products/01-expert-to-influencer-content-engine.html", hrefFr: "products/01-expert-to-influencer-content-engine-fr.html", en: "See the live product", fr: "Voir le produit live" },
        tertiary: { hrefEn: "atbc-2026-drone-surveys.html", hrefFr: "atbc-2026-drone-surveys-fr.html", en: "Read the research campaign", fr: "Lire la campagne de recherche" }
      },
      updates: {
        bodyEn: "You are on the OPLURIX update list. This is the clean path for future launches, ERIS progress, and selected biodiversity support notes.",
        bodyFr: "Vous etes sur la liste de mises a jour OPLURIX. C'est la voie propre pour les futurs lancements, l'avancement ERIS et certaines notes d'appui biodiversite.",
        detailEn: context.updateTrack ? `Track selected: ${context.updateTrack}.` : "Your update preferences have been recorded.",
        detailFr: context.updateTrack ? `Suivi selectionne : ${context.updateTrack}.` : "Vos preferences de mise a jour ont bien ete enregistrees.",
        primary: { hrefEn: "products/index.html", hrefFr: "products/index-fr.html", en: "Browse product pages", fr: "Parcourir les pages produits" },
        secondary: { href: "projects.html", en: "See the projects", fr: "Voir les projets" },
        tertiary: { href: "index.html", en: "Back to the homepage", fr: "Retour a l'accueil" }
      },
      contact: {
        bodyEn: "Your message has been recorded. If a reply is needed, follow-up will stay direct and human rather than automated or spammy.",
        bodyFr: "Votre message a bien ete enregistre. Si une reponse est necessaire, le suivi restera direct et humain plutot qu'automatise ou spammy.",
        detailEn: context.productInterest ? `Topic recorded: ${context.productInterest}.` : "Your inquiry is now in the OPLURIX contact flow.",
        detailFr: context.productInterest ? `Sujet enregistre : ${context.productInterest}.` : "Votre demande est maintenant dans le flux de contact OPLURIX.",
        primary: { href: "index.html#contact", en: "Back to contact", fr: "Retour au contact" },
        secondary: { hrefEn: "products/01-expert-to-influencer-content-engine.html", hrefFr: "products/01-expert-to-influencer-content-engine-fr.html", en: "Review the live product", fr: "Revoir le produit live" },
        tertiary: { href: "projects.html", en: "See the projects", fr: "Voir les projets" }
      },
      default: {
        bodyEn: "The form was submitted successfully. If this was an update signup, project question, or pre-sale message, you can expect a direct human follow-up when a reply is needed.",
        bodyFr: "Le formulaire a bien ete envoye. S'il s'agissait d'une inscription aux mises a jour, d'une question projet ou d'un message avant achat, vous recevrez un suivi humain direct lorsqu'une reponse est necessaire.",
        detailEn: context.sourcePage ? `Source page recorded: ${context.sourcePage}.` : "",
        detailFr: context.sourcePage ? `Page source enregistree : ${context.sourcePage}.` : "",
        primary: { href: "index.html", en: "Back to the homepage", fr: "Retour a l'accueil" },
        secondary: { href: "projects.html", en: "See the projects", fr: "Voir les projets" },
        tertiary: { href: "project-docs.html", en: "Project docs", fr: "Docs projet" }
      }
    };

    const thankYouConfig = contextMap[variant];
    localizeNodes("[data-thankyou-form-copy]", thankYouConfig.bodyEn, thankYouConfig.bodyFr);
    document.querySelectorAll("[data-thankyou-form-context]").forEach((node) => {
      const text = node.hasAttribute("data-fr") ? thankYouConfig.detailFr : thankYouConfig.detailEn;
      node.textContent = text;
      node.hidden = !text;
    });
    updateThankYouButtons(thankYouConfig);
  }

  function setupStickyMobileCta() {
    const stickyCta = document.querySelector(".sticky-mobile-cta");
    const checkoutSection = document.querySelector("#offer");

    if (!stickyCta || !checkoutSection) {
      return;
    }

    const mobileViewport = window.matchMedia("(max-width: 720px)");
    let observer = null;

    function setHidden(hidden) {
      stickyCta.classList.toggle("is-hidden", hidden);
      stickyCta.setAttribute("aria-hidden", hidden ? "true" : "false");
    }

    function refreshObserver() {
      if (observer) {
        observer.disconnect();
      }

      setHidden(false);

      if (!mobileViewport.matches || !("IntersectionObserver" in window)) {
        return;
      }

      observer = new IntersectionObserver(
        function (entries) {
          const reachedCheckout = entries.some(function (entry) {
            return entry.isIntersecting;
          });
          setHidden(reachedCheckout);
        },
        {
          rootMargin: "0px 0px -45% 0px",
          threshold: 0.18
        }
      );

      observer.observe(checkoutSection);
    }

    refreshObserver();

    if (typeof mobileViewport.addEventListener === "function") {
      mobileViewport.addEventListener("change", refreshObserver);
    } else if (typeof mobileViewport.addListener === "function") {
      mobileViewport.addListener(refreshObserver);
    }
  }

  document.addEventListener("DOMContentLoaded", function () {
    const lang = getLanguage();
    setLanguage(lang);
    hydrateTrackedForms();
    setupTrackedForms();
    wireFallbackLinks();
    if (document.querySelector(".paypal-button")) {
      loadPayPalSdk().then(function (loaded) {
        if (loaded) {
          renderPayPalButtons();
        }
      });
    }
    document.querySelectorAll("[data-set-lang]").forEach((button) => {
      button.addEventListener("click", function () {
        setLanguage(button.dataset.setLang);
        hydrateTrackedForms();
        wireFallbackLinks();
      });
    });
    updateThankYouPage();
    setupStickyMobileCta();
  });
})();
