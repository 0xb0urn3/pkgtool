// src/ui/mod.rs

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use anyhow::Result;
use std::collections::HashMap;
use crate::package_managers::{PackageManager, PackageInfo, PackageUpdate};

// Define the application state
#[derive(Default)]
pub enum InputMode {
    Normal,
    Editing,
}

// Define the application state
pub struct App {
    // UI state
    pub input_mode: InputMode,
    pub input: String,
    pub selected_tab: usize,
    pub package_list: Vec<PackageInfo>,
    pub updates_available: Vec<PackageUpdate>,
    pub error_message: Option<String>,
    // Package manager state
    package_managers: HashMap<String, Box<dyn PackageManager>>,
}

impl App {
    // Create a new application instance
    pub async fn new() -> Result<Self> {
        // Initialize with default values
        let mut app = Self {
            input_mode: InputMode::Normal,
            input: String::new(),
            selected_tab: 0,
            package_list: Vec::new(),
            updates_available: Vec::new(),
            error_message: None,
            package_managers: HashMap::new(),
        };

        // Initialize package managers
        app.initialize_package_managers().await?;

        Ok(app)
    }

    // Initialize available package managers
    async fn initialize_package_managers(&mut self) -> Result<()> {
        // Here we would detect and initialize available package managers
        // This is a simplified example
        if let Ok(apt_manager) = crate::package_managers::AptManager::new().await {
            self.package_managers.insert("apt".to_string(), Box::new(apt_manager));
        }
        
        // Add other package managers similarly
        Ok(())
    }

    // Run the application
    pub async fn run<B: Backend>(&mut self, terminal: &mut ratatui::Terminal<B>) -> Result<()> {
        loop {
            // Draw the user interface
            terminal.draw(|f| self.render(f))?;

            // Handle input events
            if let Ok(event) = crossterm::event::read() {
                match event {
                    crossterm::event::Event::Key(key) => {
                        if !self.handle_key_event(key).await? {
                            break;
                        }
                    }
                    // Handle other events as needed
                    _ => {}
                }
            }
        }
        Ok(())
    }

    // Handle keyboard input
    async fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<bool> {
        match self.input_mode {
            InputMode::Normal => match key.code {
                crossterm::event::KeyCode::Char('q') => return Ok(false),
                crossterm::event::KeyCode::Char('e') => {
                    self.input_mode = InputMode::Editing;
                }
                crossterm::event::KeyCode::Tab => {
                    self.selected_tab = (self.selected_tab + 1) % 3;
                }
                // Add other key handlers
                _ => {}
            },
            InputMode::Editing => match key.code {
                crossterm::event::KeyCode::Enter => {
                    self.handle_input().await?;
                    self.input.clear();
                    self.input_mode = InputMode::Normal;
                }
                crossterm::event::KeyCode::Char(c) => {
                    self.input.push(c);
                }
                crossterm::event::KeyCode::Backspace => {
                    self.input.pop();
                }
                crossterm::event::KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                _ => {}
            },
        }
        Ok(true)
    }

    // Handle user input
    async fn handle_input(&mut self) -> Result<()> {
        // Process the input command
        let parts: Vec<&str> = self.input.split_whitespace().collect();
        if let Some(command) = parts.first() {
            match *command {
                "search" => {
                    if let Some(query) = parts.get(1) {
                        self.search_packages(query).await?;
                    }
                }
                "update" => {
                    self.update_system().await?;
                }
                // Add other commands
                _ => {
                    self.error_message = Some("Unknown command".to_string());
                }
            }
        }
        Ok(())
    }

    // Render the user interface
    fn render<B: Backend>(&self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Top bar
                Constraint::Min(0),     // Main content
                Constraint::Length(3),  // Input bar
            ])
            .split(f.size());

        // Render top bar
        let tabs = vec!["Packages", "Updates", "Settings"];
        let tabs = ratatui::widgets::Tabs::new(tabs)
            .select(self.selected_tab)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow));
        f.render_widget(tabs, chunks[0]);

        // Render main content
        let content = match self.selected_tab {
            0 => self.render_package_list(),
            1 => self.render_updates(),
            2 => self.render_settings(),
            _ => unreachable!(),
        };
        f.render_widget(content, chunks[1]);

        // Render input bar
        let input = Paragraph::new(self.input.as_ref())
            .block(Block::default().borders(Borders::ALL))
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            });
        f.render_widget(input, chunks[2]);
    }

    // Helper methods for rendering different views
    fn render_package_list(&self) -> impl ratatui::widgets::Widget + '_ {
        let items: Vec<ratatui::widgets::ListItem> = self.package_list
            .iter()
            .map(|pkg| {
                ratatui::widgets::ListItem::new(format!("{} - {}", pkg.name, pkg.version))
            })
            .collect();

        ratatui::widgets::List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Packages"))
    }

    fn render_updates(&self) -> impl ratatui::widgets::Widget + '_ {
        let items: Vec<ratatui::widgets::ListItem> = self.updates_available
            .iter()
            .map(|update| {
                ratatui::widgets::ListItem::new(format!("{} -> {}", update.name, update.version))
            })
            .collect();

        ratatui::widgets::List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Available Updates"))
    }

    fn render_settings(&self) -> impl ratatui::widgets::Widget + '_ {
        Paragraph::new("Settings")
            .block(Block::default().borders(Borders::ALL))
    }

    // Package management methods
    async fn search_packages(&mut self, query: &str) -> Result<()> {
        for manager in self.package_managers.values() {
            if let Ok(packages) = manager.search(query).await {
                self.package_list.extend(packages);
            }
        }
        Ok(())
    }

    async fn update_system(&mut self) -> Result<()> {
        for manager in self.package_managers.values() {
            if let Ok(updates) = manager.get_updates().await {
                self.updates_available.extend(updates);
            }
        }
        Ok(())
    }
}
