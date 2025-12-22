// Portfolio - Modern Animations & Interactions
document.addEventListener('DOMContentLoaded', function() {
    // Mobile menu toggle
    const mobileMenuBtn = document.getElementById('mobile-menu-btn');
    const mobileMenu = document.getElementById('mobile-menu');
    
    if (mobileMenuBtn && mobileMenu) {
        mobileMenuBtn.addEventListener('click', function() {
            mobileMenu.classList.toggle('hidden');
            // Animate menu button
            this.querySelector('svg').classList.toggle('rotate-90');
        });
    }
    
    // Close mobile menu when clicking outside
    document.addEventListener('click', function(event) {
        if (mobileMenu && !mobileMenu.contains(event.target) && !mobileMenuBtn.contains(event.target)) {
            mobileMenu.classList.add('hidden');
        }
    });
    
    // Smooth scroll for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function(e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
    
    // Animate skill bars on scroll
    const skillBars = document.querySelectorAll('.skill-bar');
    const skillObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                const bar = entry.target;
                const targetWidth = bar.style.width;
                bar.style.width = '0%';
                setTimeout(() => {
                    bar.style.width = targetWidth;
                }, 200);
                skillObserver.unobserve(bar);
            }
        });
    }, { threshold: 0.5 });
    
    skillBars.forEach(bar => skillObserver.observe(bar));
    
    // Animate elements on scroll with stagger effect
    const animateOnScroll = () => {
        const elements = document.querySelectorAll('.animate-slide-up, .animate-scale-in, .glass-card:not(.animated)');
        
        const observer = new IntersectionObserver((entries) => {
            entries.forEach((entry, index) => {
                if (entry.isIntersecting) {
                    setTimeout(() => {
                        entry.target.style.opacity = '1';
                        entry.target.style.transform = 'translateY(0) scale(1)';
                        entry.target.classList.add('animated');
                    }, index * 100);
                    observer.unobserve(entry.target);
                }
            });
        }, { threshold: 0.1, rootMargin: '0px 0px -50px 0px' });
        
        elements.forEach(el => {
            if (!el.classList.contains('animated')) {
                el.style.opacity = '0';
                el.style.transform = 'translateY(30px)';
                el.style.transition = 'opacity 0.6s cubic-bezier(0.4, 0, 0.2, 1), transform 0.6s cubic-bezier(0.4, 0, 0.2, 1)';
                observer.observe(el);
            }
        });
    };
    
    animateOnScroll();
    
    // Navbar scroll effect
    const nav = document.querySelector('nav');
    if (nav) {
        let lastScroll = 0;
        const navHeight = nav.offsetHeight;
        
        window.addEventListener('scroll', () => {
            const currentScroll = window.pageYOffset;
            
            // Add shadow and background on scroll
            if (currentScroll > 50) {
                nav.style.background = 'rgba(10, 10, 20, 0.95)';
                nav.style.boxShadow = '0 4px 30px rgba(0, 0, 0, 0.3)';
            } else {
                nav.style.background = 'rgba(10, 10, 20, 0.7)';
                nav.style.boxShadow = 'none';
            }
            
            // Hide/show navbar on scroll
            if (currentScroll > lastScroll && currentScroll > navHeight) {
                nav.style.transform = 'translateY(-100%)';
            } else {
                nav.style.transform = 'translateY(0)';
            }
            
            lastScroll = currentScroll;
        });
        
        nav.style.transition = 'transform 0.3s ease, background 0.3s ease, box-shadow 0.3s ease';
    }
    
    // Form validation feedback
    const forms = document.querySelectorAll('form');
    forms.forEach(form => {
        const inputs = form.querySelectorAll('input, textarea');
        inputs.forEach(input => {
            input.addEventListener('focus', function() {
                this.parentElement?.classList.add('focused');
            });
            
            input.addEventListener('blur', function() {
                this.parentElement?.classList.remove('focused');
            });
            
            input.addEventListener('invalid', function(e) {
                e.target.classList.add('border-red-500');
                e.target.classList.add('shake');
                setTimeout(() => e.target.classList.remove('shake'), 500);
            });
            
            input.addEventListener('input', function(e) {
                if (e.target.validity.valid) {
                    e.target.classList.remove('border-red-500');
                    e.target.classList.add('border-green-500');
                }
            });
        });
    });
    
    // Copy to clipboard functionality
    document.querySelectorAll('[data-copy]').forEach(btn => {
        btn.addEventListener('click', async function() {
            const text = this.getAttribute('data-copy');
            try {
                await navigator.clipboard.writeText(text);
                const originalText = this.innerHTML;
                this.innerHTML = '<svg class="w-5 h-5 inline" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg> Copied!';
                this.classList.add('text-green-400');
                setTimeout(() => {
                    this.innerHTML = originalText;
                    this.classList.remove('text-green-400');
                }, 2000);
            } catch (err) {
                console.error('Failed to copy:', err);
            }
        });
    });

    // Counter animation for stats
    const animateCounters = () => {
        const counters = document.querySelectorAll('.stat-value');
        
        counters.forEach(counter => {
            const target = parseInt(counter.textContent);
            if (isNaN(target)) return;
            
            const observer = new IntersectionObserver((entries) => {
                entries.forEach(entry => {
                    if (entry.isIntersecting) {
                        let current = 0;
                        const increment = target / 50;
                        const timer = setInterval(() => {
                            current += increment;
                            if (current >= target) {
                                counter.textContent = target + '+';
                                clearInterval(timer);
                            } else {
                                counter.textContent = Math.floor(current) + '+';
                            }
                        }, 30);
                        observer.unobserve(counter);
                    }
                });
            }, { threshold: 0.5 });
            
            observer.observe(counter);
        });
    };
    
    animateCounters();
    
    // Tilt effect on cards
    const tiltCards = document.querySelectorAll('.card-3d');
    
    tiltCards.forEach(card => {
        card.addEventListener('mousemove', (e) => {
            const rect = card.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;
            
            const centerX = rect.width / 2;
            const centerY = rect.height / 2;
            
            const rotateX = (y - centerY) / 20;
            const rotateY = (centerX - x) / 20;
            
            card.style.transform = `perspective(1000px) rotateX(${rotateX}deg) rotateY(${rotateY}deg) translateY(-5px)`;
        });
        
        card.addEventListener('mouseleave', () => {
            card.style.transform = 'perspective(1000px) rotateX(0) rotateY(0) translateY(0)';
        });
    });
});

// Parallax effect for blobs
document.addEventListener('mousemove', function(e) {
    const blobs = document.querySelectorAll('.blob');
    const x = e.clientX / window.innerWidth;
    const y = e.clientY / window.innerHeight;
    
    blobs.forEach((blob, index) => {
        const speed = (index + 1) * 15;
        const xOffset = (x - 0.5) * speed;
        const yOffset = (y - 0.5) * speed;
        blob.style.transform = `translate(${xOffset}px, ${yOffset}px)`;
    });
});

// Magnetic button effect
document.querySelectorAll('.btn-primary, .btn-secondary').forEach(button => {
    button.addEventListener('mousemove', function(e) {
        const rect = this.getBoundingClientRect();
        const x = e.clientX - rect.left - rect.width / 2;
        const y = e.clientY - rect.top - rect.height / 2;
        
        this.style.transform = `translate(${x * 0.2}px, ${y * 0.2}px)`;
    });
    
    button.addEventListener('mouseleave', function() {
        this.style.transform = 'translate(0, 0)';
    });
});

// Cursor follower (optional - for desktop)
if (window.matchMedia('(pointer: fine)').matches) {
    const cursor = document.createElement('div');
    cursor.className = 'cursor-follower';
    cursor.style.cssText = `
        width: 20px;
        height: 20px;
        border: 2px solid rgba(249, 115, 22, 0.5);
        border-radius: 50%;
        position: fixed;
        pointer-events: none;
        z-index: 9999;
        transition: transform 0.15s ease, width 0.3s ease, height 0.3s ease, border-color 0.3s ease;
        transform: translate(-50%, -50%);
    `;
    document.body.appendChild(cursor);
    
    document.addEventListener('mousemove', (e) => {
        cursor.style.left = e.clientX + 'px';
        cursor.style.top = e.clientY + 'px';
    });
    
    document.querySelectorAll('a, button, [role="button"]').forEach(el => {
        el.addEventListener('mouseenter', () => {
            cursor.style.width = '40px';
            cursor.style.height = '40px';
            cursor.style.borderColor = 'rgba(249, 115, 22, 0.8)';
        });
        el.addEventListener('mouseleave', () => {
            cursor.style.width = '20px';
            cursor.style.height = '20px';
            cursor.style.borderColor = 'rgba(249, 115, 22, 0.5)';
        });
    });
}

// Lazy load images
const lazyImages = document.querySelectorAll('img[data-src]');
const imageObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            const img = entry.target;
            img.src = img.dataset.src;
            img.classList.add('loaded');
            img.removeAttribute('data-src');
            imageObserver.unobserve(img);
        }
    });
});

lazyImages.forEach(img => imageObserver.observe(img));

// Add shake animation styles
const style = document.createElement('style');
style.textContent = `
    @keyframes shake {
        0%, 100% { transform: translateX(0); }
        25% { transform: translateX(-5px); }
        75% { transform: translateX(5px); }
    }
    .shake { animation: shake 0.3s ease-in-out; }
    img.loaded { animation: scale-in 0.5s ease; }
`;
document.head.appendChild(style);

// ==========================================
// Auto Translation System (LibreTranslate)
// ==========================================
class AutoTranslator {
    constructor() {
        this.cache = JSON.parse(localStorage.getItem('translationCache') || '{}');
        this.currentLang = localStorage.getItem('selectedLang') || this.detectLanguage();
        this.isTranslating = false;
        this.translatedElements = new Set();
        // LibreTranslate endpoint - use relative URL for same-origin
        this.translateEndpoint = '/api/translate';
    }

    // Detect if user is from China or prefers Chinese
    detectLanguage() {
        const userLang = navigator.language || navigator.userLanguage;
        const isChineseUser = userLang.startsWith('zh') || 
                              userLang.includes('CN') || 
                              userLang.includes('TW') ||
                              userLang.includes('HK');
        
        // Also check timezone for China
        const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
        const isChineseTimezone = timezone.includes('Shanghai') || 
                                   timezone.includes('Hong_Kong') ||
                                   timezone.includes('Taipei');
        
        return (isChineseUser || isChineseTimezone) ? 'zh' : 'en';
    }

    // Translate text using LibreTranslate API
    async translateText(text, targetLang = 'zh') {
        if (!text || text.trim().length === 0) return text;
        if (targetLang === 'en') return text;
        
        // Check cache first
        const cacheKey = `${text.trim()}_${targetLang}`;
        if (this.cache[cacheKey]) {
            return this.cache[cacheKey];
        }

        try {
            const response = await fetch(this.translateEndpoint, {
                method: 'POST',
                headers: { 
                    'Content-Type': 'application/json',
                    'Accept': 'application/json'
                },
                body: JSON.stringify({
                    q: text.trim(),
                    source: 'en',
                    target: targetLang,
                    format: 'text'
                })
            });
            
            if (response.ok) {
                const data = await response.json();
                if (data.translatedText) {
                    // Cache the result
                    this.cache[cacheKey] = data.translatedText;
                    this.saveCache();
                    return data.translatedText;
                }
            }
        } catch (error) {
            console.warn('Translation failed:', error.message);
        }
        
        return text; // Return original if failed
    }

    saveCache() {
        try {
            // Limit cache size to prevent localStorage overflow
            const keys = Object.keys(this.cache);
            if (keys.length > 500) {
                // Remove oldest 100 entries
                keys.slice(0, 100).forEach(key => delete this.cache[key]);
            }
            localStorage.setItem('translationCache', JSON.stringify(this.cache));
        } catch (e) {
            console.warn('Cache save failed:', e);
        }
    }

    // Get all text nodes that should be translated
    getTextNodes(element) {
        const textNodes = [];
        const walker = document.createTreeWalker(
            element,
            NodeFilter.SHOW_TEXT,
            {
                acceptNode: (node) => {
                    // Skip empty nodes, scripts, styles, and already translated
                    if (!node.textContent.trim()) return NodeFilter.FILTER_REJECT;
                    if (node.parentElement.tagName === 'SCRIPT') return NodeFilter.FILTER_REJECT;
                    if (node.parentElement.tagName === 'STYLE') return NodeFilter.FILTER_REJECT;
                    if (node.parentElement.tagName === 'CODE') return NodeFilter.FILTER_REJECT;
                    if (node.parentElement.tagName === 'PRE') return NodeFilter.FILTER_REJECT;
                    if (node.parentElement.closest('[data-no-translate]')) return NodeFilter.FILTER_REJECT;
                    return NodeFilter.FILTER_ACCEPT;
                }
            }
        );
        
        let node;
        while (node = walker.nextNode()) {
            textNodes.push(node);
        }
        return textNodes;
    }

    // Translate the entire page
    async translatePage() {
        if (this.isTranslating || this.currentLang === 'en') return;
        this.isTranslating = true;
        
        this.showLoadingIndicator();
        
        try {
            const textNodes = this.getTextNodes(document.body);
            const batchSize = 5; // Translate in batches to avoid rate limiting
            
            for (let i = 0; i < textNodes.length; i += batchSize) {
                const batch = textNodes.slice(i, i + batchSize);
                await Promise.all(batch.map(async (node) => {
                    const nodeId = this.getNodeId(node);
                    if (this.translatedElements.has(nodeId)) return;
                    
                    const originalText = node.textContent.trim();
                    if (originalText.length < 2) return; // Skip very short text
                    
                    // Store original text
                    if (!node.parentElement.dataset.originalText) {
                        node.parentElement.dataset.originalText = originalText;
                    }
                    
                    const translatedText = await this.translateText(originalText, this.currentLang);
                    if (translatedText !== originalText) {
                        node.textContent = node.textContent.replace(originalText, translatedText);
                        this.translatedElements.add(nodeId);
                    }
                }));
                
                // Small delay between batches
                await new Promise(resolve => setTimeout(resolve, 100));
            }
            
            // Also translate placeholders and titles
            await this.translateAttributes();
            
        } catch (error) {
            console.error('Translation failed:', error);
        } finally {
            this.isTranslating = false;
            this.hideLoadingIndicator();
        }
    }

    // Translate element attributes (placeholders, titles, alt text)
    async translateAttributes() {
        const elements = document.querySelectorAll('[placeholder], [title], [alt]');
        
        for (const el of elements) {
            if (el.placeholder && !el.dataset.originalPlaceholder) {
                el.dataset.originalPlaceholder = el.placeholder;
                el.placeholder = await this.translateText(el.placeholder, this.currentLang);
            }
            if (el.title && !el.dataset.originalTitle) {
                el.dataset.originalTitle = el.title;
                el.title = await this.translateText(el.title, this.currentLang);
            }
            if (el.alt && !el.dataset.originalAlt) {
                el.dataset.originalAlt = el.alt;
                el.alt = await this.translateText(el.alt, this.currentLang);
            }
        }
    }

    getNodeId(node) {
        return `${node.parentElement.tagName}_${node.textContent.substring(0, 20)}`;
    }

    // Restore original text
    restoreOriginal() {
        document.querySelectorAll('[data-original-text]').forEach(el => {
            const textNode = Array.from(el.childNodes).find(n => n.nodeType === Node.TEXT_NODE);
            if (textNode) {
                textNode.textContent = el.dataset.originalText;
            }
            delete el.dataset.originalText;
        });
        
        document.querySelectorAll('[data-original-placeholder]').forEach(el => {
            el.placeholder = el.dataset.originalPlaceholder;
            delete el.dataset.originalPlaceholder;
        });
        
        document.querySelectorAll('[data-original-title]').forEach(el => {
            el.title = el.dataset.originalTitle;
            delete el.dataset.originalTitle;
        });
        
        document.querySelectorAll('[data-original-alt]').forEach(el => {
            el.alt = el.dataset.originalAlt;
            delete el.dataset.originalAlt;
        });
        
        this.translatedElements.clear();
    }

    // Set language and translate/restore
    async setLanguage(lang) {
        this.currentLang = lang;
        localStorage.setItem('selectedLang', lang);
        
        if (lang === 'en') {
            this.restoreOriginal();
        } else {
            await this.translatePage();
        }
        
        this.updateLanguageSwitcher();
    }

    // Show loading indicator
    showLoadingIndicator() {
        if (document.getElementById('translate-loading')) return;
        
        const loader = document.createElement('div');
        loader.id = 'translate-loading';
        loader.innerHTML = `
            <div style="position: fixed; top: 20px; right: 20px; z-index: 10000; 
                        background: rgba(249, 115, 22, 0.9); color: white; 
                        padding: 12px 20px; border-radius: 8px; 
                        display: flex; align-items: center; gap: 10px;
                        box-shadow: 0 4px 15px rgba(0,0,0,0.2);">
                <svg class="animate-spin" style="width: 20px; height: 20px;" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle style="opacity: 0.25;" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path style="opacity: 0.75;" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                <span>翻译中... Translating...</span>
            </div>
        `;
        document.body.appendChild(loader);
    }

    hideLoadingIndicator() {
        const loader = document.getElementById('translate-loading');
        if (loader) loader.remove();
    }

    updateLanguageSwitcher() {
        const buttons = document.querySelectorAll('.lang-btn');
        buttons.forEach(btn => {
            btn.classList.remove('active');
            if (btn.dataset.lang === this.currentLang) {
                btn.classList.add('active');
            }
        });
    }

    // Create language switcher UI
    createLanguageSwitcher() {
        const switcher = document.createElement('div');
        switcher.id = 'language-switcher';
        switcher.innerHTML = `
            <style>
                #language-switcher {
                    position: fixed;
                    top: 80px;
                    right: 20px;
                    z-index: 9999;
                    display: flex;
                    gap: 4px;
                    background: rgba(30, 30, 46, 0.9);
                    backdrop-filter: blur(10px);
                    padding: 4px;
                    border-radius: 8px;
                    border: 1px solid rgba(255,255,255,0.1);
                    box-shadow: 0 4px 15px rgba(0,0,0,0.2);
                }
                .lang-btn {
                    padding: 8px 16px;
                    border: none;
                    border-radius: 6px;
                    cursor: pointer;
                    font-weight: 600;
                    font-size: 14px;
                    transition: all 0.3s ease;
                    background: transparent;
                    color: #9ca3af;
                }
                .lang-btn:hover {
                    color: white;
                    background: rgba(249, 115, 22, 0.2);
                }
                .lang-btn.active {
                    background: linear-gradient(135deg, #f97316, #ea580c);
                    color: white;
                    box-shadow: 0 2px 10px rgba(249, 115, 22, 0.3);
                }
                @keyframes spin {
                    from { transform: rotate(0deg); }
                    to { transform: rotate(360deg); }
                }
                .animate-spin {
                    animation: spin 1s linear infinite;
                }
            </style>
            <button class="lang-btn ${this.currentLang === 'en' ? 'active' : ''}" data-lang="en">EN</button>
            <button class="lang-btn ${this.currentLang === 'zh' ? 'active' : ''}" data-lang="zh">中文</button>
        `;
        
        document.body.appendChild(switcher);
        
        // Add click handlers
        switcher.querySelectorAll('.lang-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                this.setLanguage(btn.dataset.lang);
            });
        });
    }

    // Initialize
    async init() {
        this.createLanguageSwitcher();
        
        // Auto-translate if Chinese user
        if (this.currentLang === 'zh') {
            // Wait for page to fully load
            if (document.readyState === 'complete') {
                await this.translatePage();
            } else {
                window.addEventListener('load', () => this.translatePage());
            }
        }
        
        // Watch for dynamic content changes
        this.observeDynamicContent();
    }

    // Watch for dynamic content and translate it
    observeDynamicContent() {
        const observer = new MutationObserver((mutations) => {
            if (this.currentLang !== 'zh' || this.isTranslating) return;
            
            let hasNewContent = false;
            mutations.forEach(mutation => {
                if (mutation.addedNodes.length > 0) {
                    hasNewContent = true;
                }
            });
            
            if (hasNewContent) {
                // Debounce translation for dynamic content
                clearTimeout(this.dynamicTranslateTimeout);
                this.dynamicTranslateTimeout = setTimeout(() => {
                    this.translatePage();
                }, 500);
            }
        });
        
        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    }
}

// Initialize translator
const autoTranslator = new AutoTranslator();
autoTranslator.init();
