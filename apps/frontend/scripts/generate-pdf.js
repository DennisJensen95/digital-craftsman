const puppeteer = require("puppeteer");
const path = require("path");
const fs = require("fs");

async function generatePDF() {
  console.log("Starting PDF generation...");

  const browser = await puppeteer.launch({
    headless: true,
    args: [
      "--no-sandbox",
      "--disable-setuid-sandbox",
      "--disable-dev-shm-usage",
      "--disable-gpu",
      "--disable-web-security",
      "--allow-running-insecure-content",
    ],
  });

  try {
    const page = await browser.newPage();

    // Set viewport for consistent rendering
    await page.setViewport({ width: 1200, height: 1600 });

    // Add compact PDF optimization CSS
    await page.addStyleTag({
      content: `
        @media print {
          * {
            -webkit-print-color-adjust: exact !important;
            color-adjust: exact !important;
            print-color-adjust: exact !important;
            box-shadow: none !important;
            border-radius: 0 !important;
          }

          body {
            background: white !important;
            margin: 0 !important;
            padding: 0 !important;
          }

          .navigation, .nav-container, .chat-container,
          button, input, .input-container, .example-inputs,
          .nav-item, .social-links, .header-container,
          .container > div[style], hr {
            display: none !important;
          }

          .container {
            box-shadow: none !important;
            border: none !important;
            margin: 0 !important;
            padding: 0 !important;
            width: 100% !important;
            max-width: none !important;
            background: transparent !important;
          }

          /* Ensure CV component is visible and properly styled */
          app-cv,
          .cv-container {
            display: block !important;
            visibility: visible !important;
          }
        }
      `,
    });

    console.log("Navigating to resume page...");

    // Navigate to the unified CV page
    await page.goto("http://localhost:8080/cv", {
      waitUntil: "networkidle0",
      timeout: 30000,
    });

    // Wait for content to fully load
    await new Promise((resolve) => setTimeout(resolve, 3000));

    // Wait for any dynamic content and fonts
    await page.evaluate(() => {
      return new Promise((resolve) => {
        if (document.readyState === "complete" && document.fonts.ready) {
          document.fonts.ready.then(() => resolve());
        } else {
          window.addEventListener("load", () => {
            document.fonts.ready.then(() => resolve());
          });
        }
      });
    });

    // Ensure all images are loaded
    await page.evaluate(() => {
      const images = Array.from(document.images);
      return Promise.all(
        images.map((img) => {
          if (img.complete) return Promise.resolve();
          return new Promise((resolve) => {
            img.addEventListener("load", resolve);
            img.addEventListener("error", resolve);
          });
        }),
      );
    });

    console.log("Generating PDF...");

    // Generate compact single-page PDF
    const pdfBuffer = await page.pdf({
      format: "A4",
      printBackground: true,
      preferCSSPageSize: false,
      scale: 0.9,
      margin: {
        top: "1.5cm",
        right: "2cm",
        bottom: "1.5cm",
        left: "2cm",
      },
      displayHeaderFooter: false,
      tagged: true,
      outline: false,
    });

    // Save PDF with timestamp
    const timestamp = new Date().toISOString().slice(0, 10);
    const filename = `cv-dennis-jensen-${timestamp}.pdf`;

    fs.writeFileSync(filename, pdfBuffer);

    console.log(`✅ PDF generated successfully: ${filename}`);
    console.log(`📄 File saved in: ${path.resolve(filename)}`);

    // Also create a generic version without timestamp
    fs.writeFileSync("cv-dennis-jensen.pdf", pdfBuffer);
    console.log(`📄 Generic version saved: cv-dennis-jensen.pdf`);
  } catch (error) {
    console.error("❌ Error generating PDF:", error);
    throw error;
  } finally {
    await browser.close();
  }
}

// Check if server is running
async function checkServer() {
  const http = require("http");
  return new Promise((resolve) => {
    const req = http.get("http://localhost:8080", (res) => {
      resolve(res.statusCode === 200);
    });
    req.on("error", () => {
      resolve(false);
    });
    req.setTimeout(5000, () => {
      req.destroy();
      resolve(false);
    });
  });
}

async function main() {
  console.log("🔍 Checking if Angular dev server is running...");

  const serverRunning = await checkServer();

  if (!serverRunning) {
    console.log(
      "❌ Angular dev server is not running on http://localhost:4200",
    );
    console.log("📋 Please run the following commands first:");
    console.log("   1. npm run build");
    console.log("   2. npm run start");
    console.log("   3. Wait for the server to start");
    console.log("   4. Then run this script again");
    process.exit(1);
  }

  console.log("✅ Server is running, proceeding with PDF generation...");
  await generatePDF();
}

main().catch(console.error);
