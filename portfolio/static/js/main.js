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
