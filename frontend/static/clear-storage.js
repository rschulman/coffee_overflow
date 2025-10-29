// Clear corrupted localStorage on load
try {
  const stored = localStorage.getItem('user');
  if (stored && stored !== 'undefined' && stored !== '') {
    JSON.parse(stored);
  }
} catch (e) {
  console.log('Clearing corrupted user data from localStorage');
  localStorage.removeItem('user');
}
