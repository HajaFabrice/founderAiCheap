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
        payhipUrl: "https://payhip.com/b/6I1p4"
      },
      "ecor-starter": {
        id: "ecor-starter",
        nameEn: "EcoR Toolkit Starter",
        nameFr: "EcoR Toolkit Starter",
        priceUsd: 29,
        checkoutEnabled: true
      },
      "ecor-complete": {
        id: "ecor-complete",
        nameEn: "EcoR Toolkit Complete",
        nameFr: "EcoR Toolkit Complete",
        priceUsd: 97,
        checkoutEnabled: true
      },
      "ecor-pro": {
        id: "ecor-pro",
        nameEn: "EcoR Toolkit Pro Bundle",
        nameFr: "EcoR Toolkit Pro Bundle",
        priceUsd: 297,
        checkoutEnabled: true
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
      "biodiversity-pitch-deck-builder": {
        id: "biodiversity-pitch-deck-builder",
        nameEn: "Biodiversity Pitch Deck Builder",
        nameFr: "Biodiversity Pitch Deck Builder",
        priceUsd: 69,
        checkoutEnabled: false
      }
    }
  };

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

  function wirePayhipButtons(lang) {
    document.querySelectorAll(".paypal-button").forEach((button) => {
      const product = store.products[button.dataset.paypalProduct];
      if (!product || !product.payhipUrl) {
        return;
      }
      const note = document.querySelector(`[data-product-note="${button.dataset.paypalProduct}"]`);
      button.href = product.payhipUrl;
      button.target = "_blank";
      button.rel = "noopener";
      button.textContent =
        lang === "fr"
          ? `Acheter maintenant — $${product.priceUsd} via Payhip`
          : `Buy now — $${product.priceUsd} via Payhip`;
      if (note) {
        note.textContent =
          lang === "fr"
            ? "Paiement securise via Payhip. Livraison automatique par email apres paiement."
            : "Secure checkout via Payhip. Automatic delivery by email after payment.";
      }
    });
  }

  function wireFallbackLinks() {
    const lang = document.body.dataset.lang || getLanguage();
    document.querySelectorAll(".paypal-button").forEach((button) => {
      const product = store.products[button.dataset.paypalProduct];
      if (!product || product.payhipUrl) {
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
      if (!product || product.payhipUrl) {
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
  }

  document.addEventListener("DOMContentLoaded", function () {
    const lang = getLanguage();
    setLanguage(lang);
    wirePayhipButtons(lang);
    wireFallbackLinks();
    loadPayPalSdk().then(function (loaded) {
      if (loaded) {
        renderPayPalButtons();
      }
    });
    document.querySelectorAll("[data-set-lang]").forEach((button) => {
      button.addEventListener("click", function () {
        const newLang = button.dataset.setLang;
        setLanguage(newLang);
        wirePayhipButtons(newLang);
        wireFallbackLinks();
      });
    });
    updateThankYouPage();
  });
})();
